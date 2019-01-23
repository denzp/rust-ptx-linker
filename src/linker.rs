use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

use llvm_sys::bit_reader::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::linker::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::{ipo::*, pass_manager_builder::*};

use ar::Archive;
use failure::{bail, Error, ResultExt};
use log::*;

use crate::error::*;
use crate::llvm::{Message, PassRunner};
use crate::passes::{
    FindExternalReferencesPass, InternalizePass, RenameFunctionsPass, RenameGlobalsPass,
};
use crate::session::{OptLevel, Output, Session};

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

    pub fn link(self) -> Result<(), Error> {
        info!(
            "Going to link {} bitcode modules and {} rlibs...",
            self.session.include_bitcode_modules.len(),
            self.session.include_rlibs.len()
        );

        self.link_bitcode()?;
        self.link_rlibs()?;

        self.run_passes()?;
        self.run_llvm_passes();

        for output in &self.session.emit {
            match *output {
                Output::PTXAssembly => self.emit_asm().context("Unable to emit PTX assembly")?,
                Output::Bitcode => self.emit_bc().context("Unable to emit LLVM bitcode")?,

                Output::IntermediateRepresentation => {
                    self.emit_ir().context("Unable to emit LLVM IR code")?
                }
            }
        }

        Ok(())
    }

    fn link_bitcode(&self) -> Result<(), Error> {
        for module_path in &self.session.include_bitcode_modules {
            debug!("Linking bitcode: {:?}", module_path);

            let mut bitcode_file = BufReader::new(File::open(&module_path)?);
            let mut bitcode_bytes = vec![];

            bitcode_file.read_to_end(&mut bitcode_bytes)?;
            self.link_bitcode_contents(self.module, bitcode_bytes)?;
        }

        Ok(())
    }

    fn link_rlibs(&self) -> Result<(), Error> {
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

    fn run_passes(&self) -> Result<(), Error> {
        let runner = PassRunner::new(self.module);

        let mut internalize_pass = InternalizePass::new();
        runner.run_functions_visitor(&mut internalize_pass);
        runner.run_globals_visitor(&mut internalize_pass);

        let mut external_references_pass = FindExternalReferencesPass::new();
        runner.run_calls_visitor(&mut external_references_pass);

        if external_references_pass.count() > 0 {
            bail!(LinkerError::UndefinedReferences(
                external_references_pass.references()
            ));
        }

        // TODO(denzp): the two passes will become obsolete with built-in target.
        runner.run_globals_visitor(&mut RenameGlobalsPass::new());
        runner.run_functions_visitor(&mut RenameFunctionsPass::new());

        Ok(())
    }

    fn run_llvm_passes(&self) {
        unsafe {
            let builder = LLVMPassManagerBuilderCreate();
            let pass_manager = LLVMCreatePassManager();

            match self.session.opt_level {
                OptLevel::None => {
                    info!("Linking without optimisations");
                    LLVMPassManagerBuilderSetOptLevel(builder, 0);
                }

                OptLevel::Default => {
                    info!("Linking with Link Time Optimisation");
                    LLVMPassManagerBuilderSetOptLevel(builder, 3);
                    LLVMPassManagerBuilderPopulateLTOPassManager(builder, pass_manager, 1, 1);
                }
            }

            LLVMPassManagerBuilderPopulateModulePassManager(builder, pass_manager);
            LLVMPassManagerBuilderDispose(builder);

            LLVMAddGlobalDCEPass(pass_manager);
            LLVMRunPassManager(pass_manager, self.module);
            LLVMDisposePassManager(pass_manager);

            if self.session.debug_info {
                // Temporary workaround until https://reviews.llvm.org/D46189 is ready
                warn!("Removing debug info because it's not yet supported.");
                LLVMStripModuleDebugInfo(self.module);
            } else {
                LLVMStripModuleDebugInfo(self.module);
            }

            // TODO(denzp): this will become obsolete with built-in target.
            let inline_asm_contents = CString::new(vec![]).unwrap();
            LLVMSetModuleInlineAsm2(self.module, inline_asm_contents.as_ptr(), 0);
        }
    }

    fn emit_ir(&self) -> Result<(), Error> {
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

    fn emit_bc(&self) -> Result<(), Error> {
        let path = CString::new(self.get_output_path_with_ext("bc")?.to_str().unwrap()).unwrap();

        unsafe {
            // TODO: check result
            LLVMWriteBitcodeToFile(self.module, path.as_ptr());
        }

        info!("LLVM bitcode has been written to {:?}", path);
        Ok(())
    }

    fn emit_asm(&self) -> Result<(), Error> {
        if self.session.achitectures.len() > 1 {
            bail!("More than 1 CUDA architecture is not yet supported with PTX output.");
        }

        // TOOD(denzp): is it possible to get architecture coming from Rust?
        let arch = match self.session.achitectures.iter().next() {
            Some(arch) => &arch,
            None => "sm_20",
        };

        let path = CString::new(self.get_output_path()?.to_str().unwrap()).unwrap();
        let arch = CString::new(arch).unwrap();
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
                arch.as_ptr(),
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
                    path.as_ptr() as *mut _,
                    LLVMCodeGenFileType::LLVMAssemblyFile,
                    message.as_mut_ptr(),
                );
            }

            LLVMDisposeTargetMachine(target_machine);
        }

        info!("PTX assembly has been written to {:?}", path);
        Ok(())
    }

    fn get_output_path(&self) -> Result<PathBuf, Error> {
        match self.session.output.as_ref() {
            Some(path) => Ok(path.clone()),
            None => bail!(LinkerError::NoOutputPathError),
        }
    }

    fn get_output_path_with_ext(&self, extension: &str) -> Result<PathBuf, Error> {
        let mut output_path = self.get_output_path()?;
        output_path.set_extension(extension);

        Ok(output_path)
    }

    fn link_bitcode_contents(&self, module: LLVMModuleRef, buffer: Vec<u8>) -> Result<(), Error> {
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
