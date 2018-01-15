use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::ffi::CString;
use std::str;
use std::ptr;

use ar::Archive;
use cty::{c_char, c_uint};
use llvm;
use error::*;
use session::{Configuration, Output, Session};

pub struct Linker {
    session: Session,
    context: llvm::ContextRef,
    module: llvm::ModuleRef,
}

impl Linker {
    pub fn new(session: Session) -> Self {
        let module_name = CString::new("nvptx-module").unwrap();
        let context = unsafe { llvm::LLVMContextCreate() };

        Linker {
            session,
            context,
            module: unsafe {
                llvm::LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context)
            },
        }
    }

    pub fn link(self) -> Result<()> {
        self.link_bitcode();
        self.link_rlibs();
        self.run_passes();

        unsafe {
            llvm::StripInternalFunctions(self.module);
            llvm::RenameGlobalVariables(self.module);

            let mut message = llvm::Message::new();
            if llvm::FindExternalReferences(self.module, &mut message) > 0 {
                let references: Vec<String> = message
                    .to_string()
                    .split(";")
                    .map(|name| String::from(name))
                    .collect();

                return Err(ErrorKind::UndefinedReferences(references).into());
            }
        }

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

    fn link_bitcode(&self) {
        for module_path in &self.session.include_bitcode_modules {
            debug!("Linking bitcode: {:?}", module_path);

            let mut bitcode_file = BufReader::new(File::open(&module_path).unwrap());
            let mut bitcode_bytes = vec![];

            bitcode_file.read_to_end(&mut bitcode_bytes).unwrap();
            self.link_bitcode_contents(self.module, bitcode_bytes)
                .unwrap();
        }
    }

    fn link_rlibs(&self) {
        for rlib_path in &self.session.include_rlibs {
            debug!("Linking rlib: {:?}", rlib_path);

            let archive_reader = File::open(rlib_path).unwrap();
            let mut archive = Archive::new(archive_reader);

            while let Some(Ok(mut item)) = archive.next_entry() {
                let name = PathBuf::from(str::from_utf8(item.header().identifier()).unwrap());

                if self.is_rlib_item_linkable(&name) {
                    debug!("  - linking archive item: {:?}", name);

                    let mut bitcode_bytes = vec![];
                    item.read_to_end(&mut bitcode_bytes).unwrap();
                    self.link_bitcode_contents(self.module, bitcode_bytes)
                        .unwrap();
                }
            }
        }
    }

    fn is_rlib_item_linkable(&self, name: &Path) -> bool {
        name.extension().unwrap() == "o"
    }

    fn run_passes(&self) {
        unsafe {
            let builder = llvm::LLVMPassManagerBuilderCreate();
            let pass_manager = llvm::LLVMCreatePassManager();

            match self.session.configuration {
                Configuration::Debug => {
                    info!("Linking without optimisations");
                    llvm::LLVMPassManagerBuilderSetOptLevel(builder, 0);
                }

                Configuration::Release => {
                    info!("Linking with Link Time Optimisation");
                    llvm::LLVMPassManagerBuilderSetOptLevel(builder, 3);
                    llvm::LLVMPassManagerBuilderPopulateLTOPassManager(
                        builder,
                        pass_manager,
                        llvm::True,
                        llvm::True,
                    );
                }
            }

            llvm::LLVMPassManagerBuilderPopulateModulePassManager(builder, pass_manager);
            llvm::LLVMPassManagerBuilderDispose(builder);

            llvm::LLVMRunPassManager(pass_manager, self.module);
            llvm::LLVMDisposePassManager(pass_manager);
        }
    }

    fn emit_ir(&self) -> Result<()> {
        let path = CString::new(self.get_output_path_with_ext("ll")?.to_str().unwrap()).unwrap();

        unsafe {
            // TODO: check result
            let mut message = llvm::Message::new();
            llvm::LLVMPrintModuleToFile(self.module, path.as_ptr(), &mut message);

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
            llvm::LLVMWriteBitcodeToFile(self.module, path.as_ptr());
        }

        info!("LLVM bitcode has been written to {:?}", path);
        Ok(())
    }

    fn emit_asm(&self) -> Result<()> {
        let path = CString::new(self.get_output_path()?.to_str().unwrap()).unwrap();

        let cpu = CString::new("sm_20").unwrap();
        let feature = CString::new("").unwrap();

        unsafe {
            llvm::LLVMInitializeNVPTXTargetInfo();
            llvm::LLVMInitializeNVPTXTarget();
            llvm::LLVMInitializeNVPTXTargetMC();
            llvm::LLVMInitializeNVPTXAsmPrinter();

            let triple = llvm::LLVMGetTarget(self.module);

            let target = {
                let mut target = ptr::null_mut();
                let mut message = llvm::Message::new();

                match llvm::LLVMGetTargetFromTriple(triple, &mut target, &mut message) {
                    0 => target,
                    _ => bail!("Unable to find the target: {}", message),
                }
            };

            let target_machine = llvm::LLVMCreateTargetMachine(
                target,
                triple,
                cpu.as_ptr(),
                feature.as_ptr(),
                llvm::CodeGenOptLevel::Default,
                llvm::RelocMode::Default,
                llvm::CodeModel::Default,
            );

            {
                let mut message = llvm::Message::new();

                // TODO: check result
                llvm::LLVMTargetMachineEmitToFile(
                    target_machine,
                    self.module,
                    path.as_ptr(),
                    llvm::CodeGenFileType::AssemblyFile,
                    &mut message,
                );
            }

            llvm::LLVMDisposeTargetMachine(target_machine);
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

    fn link_bitcode_contents(&self, module: llvm::ModuleRef, buffer: Vec<u8>) -> Result<()> {
        unsafe {
            let buffer_name = CString::new("sm_20").unwrap();
            let buffer = llvm::LLVMCreateMemoryBufferWithMemoryRange(
                buffer.as_ptr() as *const c_char,
                buffer.len() as c_uint,
                buffer_name.as_ptr(),
                llvm::False,
            );

            let mut temp_module = ptr::null_mut();

            // TODO: check result
            llvm::LLVMParseBitcodeInContext2(self.context, buffer, &mut temp_module);

            // TODO: check result
            llvm::LLVMLinkModules2(module, temp_module);

            llvm::LLVMDisposeMemoryBuffer(buffer);
        }

        Ok(())
    }
}

impl Drop for Linker {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeModule(self.module);
            llvm::LLVMContextDispose(self.context);
        }
    }
}
