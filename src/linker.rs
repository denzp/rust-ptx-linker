use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

use ar::Archive;
use llvm_sys::bit_reader::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::linker::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;

use error::*;
use llvm::{Message, PassRunner};
use passes::{
    FindExternalReferencesPass, FindInternalFunctionsPass, FindInternalGlobalsPass,
    RenameFunctionsPass, RenameGlobalsPass,
};
use session::{Configuration, Output, Session};

pub struct Linker {
    session: Session,
    context: LLVMContextRef,
    module: LLVMModuleRef,
}

impl Linker {
    pub fn new(session: Session) -> Self {
        let module_name = CString::new("nvptx-module").unwrap();
        let context = unsafe { LLVMContextCreate() };

        Linker {
            session,
            context,
            module: unsafe { LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context) },
        }
    }

    pub fn link(self) -> Result<()> {
        info!(
            "Going to link {} bitcode modules and {} rlibs...\n",
            self.session.include_bitcode_modules.len(),
            self.session.include_rlibs.len()
        );

        self.link_bitcode()?;
        self.link_rlibs()?;

        self.run_llvm_passes();
        self.run_passes()?;

        for output in &self.session.emit {
            match *output {
                Output::PTXAssembly => self.emit_asm().chain_err(|| "Unable to emit PTX assembly")?,
                Output::Bitcode => self.emit_bc().chain_err(|| "Unable to emit LLVM bitcode")?,
                Output::IntermediateRepresentation => {
                    self.emit_ir().chain_err(|| "Unable to emit LLVM IR code")?
                }
            }
        }

        Ok(())
    }

    fn link_bitcode(&self) -> Result<()> {
        for module_path in &self.session.include_bitcode_modules {
            debug!("Linking bitcode: {:?}", module_path);

            let mut bitcode_file = BufReader::new(File::open(&module_path)?);
            let mut bitcode_bytes = vec![];

            bitcode_file.read_to_end(&mut bitcode_bytes)?;
            self.link_bitcode_contents(self.module, bitcode_bytes)?;
        }

        Ok(())
    }

    fn link_rlibs(&self) -> Result<()> {
        for rlib_path in &self.session.include_rlibs {
            debug!("Linking rlib: {:?}", rlib_path);

            let archive_reader = File::open(rlib_path)?;
            let mut archive = Archive::new(archive_reader);

            while let Some(Ok(mut item)) = archive.next_entry() {
                let name = PathBuf::from(str::from_utf8(item.header().identifier()).unwrap());

                if self.is_rlib_item_linkable(&name) {
                    debug!("  - linking archive item: {:?}", name);

                    let mut bitcode_bytes = vec![];
                    item.read_to_end(&mut bitcode_bytes)?;

                    self.link_bitcode_contents(self.module, bitcode_bytes)?;
                }
            }
        }

        Ok(())
    }

    fn is_rlib_item_linkable(&self, name: &Path) -> bool {
        name.extension().unwrap() == "o"
    }

    fn run_passes(&self) -> Result<()> {
        let runner = unsafe { PassRunner::new(::std::mem::transmute(self.module)) };

        let mut find_internal_functions_pass = FindInternalFunctionsPass::new();
        runner.run_calls_visitor(&mut find_internal_functions_pass);
        runner.run_functions_visitor(&mut find_internal_functions_pass.into_remove_pass());

        let mut find_internal_globals_pass = FindInternalGlobalsPass::new();
        runner.run_globals_visitor(&mut find_internal_globals_pass);
        runner.run_module_visitor(&mut find_internal_globals_pass.into_remove_pass());

        let mut external_references_pass = FindExternalReferencesPass::new();
        runner.run_calls_visitor(&mut external_references_pass);

        if external_references_pass.count() > 0 {
            return Err(
                ErrorKind::UndefinedReferences(external_references_pass.references()).into(),
            );
        }

        runner.run_globals_visitor(&mut RenameGlobalsPass::new());
        runner.run_functions_visitor(&mut RenameFunctionsPass::new());

        Ok(())
    }

    fn run_llvm_passes(&self) {
        unsafe {
            let builder = LLVMPassManagerBuilderCreate();
            let pass_manager = LLVMCreatePassManager();

            match self.session.configuration {
                Configuration::Debug => {
                    info!("Linking without optimisations");
                    LLVMPassManagerBuilderSetOptLevel(builder, 0);
                }

                Configuration::Release => {
                    info!("Linking with Link Time Optimisation");
                    LLVMPassManagerBuilderSetOptLevel(builder, 3);
                    LLVMPassManagerBuilderPopulateLTOPassManager(builder, pass_manager, 1, 1);
                }
            }

            LLVMPassManagerBuilderPopulateModulePassManager(builder, pass_manager);
            LLVMPassManagerBuilderDispose(builder);

            LLVMRunPassManager(pass_manager, self.module);
            LLVMDisposePassManager(pass_manager);

            LLVMSetModuleInlineAsm(self.module, CString::new(vec![]).unwrap().as_ptr());
        }
    }

    fn emit_ir(&self) -> Result<()> {
        let path = CString::new(self.get_output_path_with_ext("ll")?.to_str().unwrap()).unwrap();

        unsafe {
            // TODO: check result
            let mut message = Message::new();
            LLVMPrintModuleToFile(self.module, path.as_ptr(), message.as_mut_ptr());

            if !message.is_empty() {
                // TODO: stderr?
                println!("{}", message);
            }
        }

        info!("LLVM IR has been written to {:?}", path);
        Ok(())
    }

    fn emit_bc(&self) -> Result<()> {
        let path = CString::new(self.get_output_path_with_ext("bc")?.to_str().unwrap()).unwrap();

        unsafe {
            // TODO: check result
            LLVMWriteBitcodeToFile(self.module, path.as_ptr());
        }

        info!("LLVM bitcode has been written to {:?}", path);
        Ok(())
    }

    fn emit_asm(&self) -> Result<()> {
        let path = CString::new(self.get_output_path()?.to_str().unwrap()).unwrap();

        // TODO: get `cpu` from current module
        let cpu = CString::new("sm_20").unwrap();
        let feature = CString::new("").unwrap();

        unsafe {
            LLVMInitializeNVPTXTargetInfo();
            LLVMInitializeNVPTXTarget();
            LLVMInitializeNVPTXTargetMC();
            LLVMInitializeNVPTXAsmPrinter();

            let triple = LLVMGetTarget(self.module);

            let target = {
                let mut target = ptr::null_mut();
                let mut message = Message::new();

                match LLVMGetTargetFromTriple(triple, &mut target, message.as_mut_ptr()) {
                    0 => target,
                    _ => bail!("Unable to find the target: {}", message),
                }
            };

            let target_machine = LLVMCreateTargetMachine(
                target,
                triple,
                cpu.as_ptr(),
                feature.as_ptr(),
                LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive, // TODO: investigate about right settings
                LLVMRelocMode::LLVMRelocDefault,
                LLVMCodeModel::LLVMCodeModelDefault,
            );

            {
                let mut message = Message::new();

                // TODO: check result
                LLVMTargetMachineEmitToFile(
                    target_machine,
                    self.module,
                    ::std::mem::transmute(path.as_ptr()),
                    LLVMCodeGenFileType::LLVMAssemblyFile,
                    message.as_mut_ptr(),
                );
            }

            LLVMDisposeTargetMachine(target_machine);
        }

        info!("PTX assembly has been written to {:?}", path);
        Ok(())
    }

    fn get_output_path(&self) -> Result<PathBuf> {
        match self.session.output.as_ref() {
            Some(path) => Ok(path.clone()),
            None => Err(ErrorKind::NoOutputPathError.into()),
        }
    }

    fn get_output_path_with_ext(&self, extension: &str) -> Result<PathBuf> {
        let mut output_path = self.get_output_path()?;
        output_path.set_extension(extension);

        Ok(output_path)
    }

    fn link_bitcode_contents(&self, module: LLVMModuleRef, buffer: Vec<u8>) -> Result<()> {
        unsafe {
            let buffer_name = CString::new("sm_20").unwrap();
            let buffer = LLVMCreateMemoryBufferWithMemoryRange(
                buffer.as_ptr() as *const i8,
                buffer.len() as usize,
                buffer_name.as_ptr(),
                0,
            );

            let mut temp_module = ptr::null_mut();

            // TODO: check result
            LLVMParseBitcodeInContext2(self.context, buffer, &mut temp_module);

            // TODO: check result
            LLVMLinkModules2(module, temp_module);
            LLVMDisposeMemoryBuffer(buffer);
        }

        Ok(())
    }
}

impl Drop for Linker {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
            LLVMContextDispose(self.context);
        }
    }
}
