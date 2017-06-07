pub use rustc_llvm::*;

// TODO: use `cty` create here
extern "C" {
    pub fn LLVMModuleCreateWithName(id: *const i8) -> ModuleRef; // const char*
    pub fn LLVMGetTarget(module: ModuleRef) -> *const i8; // const char*
    pub fn LLVMPrintModuleToFile(module: ModuleRef, file_path: *const i8, TODO: *const i8) -> bool; // TODO: messages

    pub fn LLVMPassManagerBuilderSetOptLevel (builder: PassManagerBuilderRef, opt_level: u32); // TODO: unsigned is not really u32 here...
    pub fn LLVMAddStripDeadPrototypesPass(manager: PassManagerRef);
    pub fn LLVMAddStripSymbolsPass(manager: PassManagerRef);

    pub fn LLVMInitializeNVPTXTargetInfo();
    pub fn LLVMInitializeNVPTXTarget();
    pub fn LLVMInitializeNVPTXTargetMC();
    pub fn LLVMInitializeNVPTXAsmPrinter();
}

