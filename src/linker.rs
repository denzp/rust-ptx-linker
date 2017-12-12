use std::fs::File;
use std::io::{Read, BufReader};
use std::path::{Path, PathBuf};
use std::ffi::CString;

use rustc_llvm::archive_ro::ArchiveRO;
use cty::c_char;
use llvm;
use error::*;
use session::{Session, Output, Configuration};

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
            unsafe {
                // TODO: check result
                llvm::LLVMRustLinkInExternalBitcode(self.module,
                                                    bitcode_bytes.as_ptr() as *const c_char,
                                                    bitcode_bytes.len());
            }
        }
    }

    fn link_rlibs(&self) {
        for rlib_path in &self.session.include_rlibs {
            debug!("Linking rlib: {:?}", rlib_path);

            let archive = ArchiveRO::open(rlib_path).unwrap();
            for item in archive.iter() {
                let name = Path::new(item.as_ref().unwrap().name().unwrap());

                if self.is_rlib_item_linkable(&name) {
                    debug!("  - linking archive item: {:?}", name);
                    let data = item.as_ref().unwrap().data();

                    unsafe {
                        // TODO: check result
                        llvm::LLVMRustLinkInExternalBitcode(self.module,
                                                            data.as_ptr() as *const c_char,
                                                            data.len());
                    }
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
                    llvm::LLVMPassManagerBuilderPopulateLTOPassManager(builder,
                                                                       pass_manager,
                                                                       llvm::True,
                                                                       llvm::True);
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
            let target = llvm::LLVMRustCreateTargetMachine(triple,
                                                           cpu.as_ptr(),
                                                           feature.as_ptr(),
                                                           llvm::CodeModel::Default,
                                                           llvm::RelocMode::Default,
                                                           llvm::CodeGenOptLevel::Default,
                                                           false,
                                                           false,
                                                           false,
                                                           false,
                                                           true,
                                                           true);

            // TODO: check `target` != nullptr

            // We need an empty pass manager only for LLVM to add "asm printer" pass.
            let emitting_pass_manager = llvm::LLVMCreatePassManager();

            // TODO: check result
            llvm::LLVMRustWriteOutputFile(target,
                                          emitting_pass_manager,
                                          self.module,
                                          path.as_ptr(),
                                          llvm::FileType::AssemblyFile);

            llvm::LLVMRustDisposeTargetMachine(target);

            // We don't need to dispose `emiting_pass_manager` because Rust C api is already did this:
            // https://github.com/rust-lang/rust/blob/119066ff2bb39f7c8f7d1e68b7ad15e026f048e2/src/rustllvm/PassWrapper.cpp#L502
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
}

impl Drop for Linker {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMDisposeModule(self.module);
            llvm::LLVMContextDispose(self.context);
        }
    }
}

