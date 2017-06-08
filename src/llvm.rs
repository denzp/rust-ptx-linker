use cty::{c_char, c_uint};

pub use rustc_llvm::*;

extern "C" {
    pub fn LLVMModuleCreateWithName(id: *const c_char) -> ModuleRef;
    pub fn LLVMGetTarget(module: ModuleRef) -> *const c_char;

    pub fn LLVMDisposeMessage(message: *const c_char);
    pub fn LLVMPrintModuleToFile(module: ModuleRef,
                                 file_path: *const c_char,
                                 message_ptr: &mut *const c_char)
                                 -> bool;

    pub fn LLVMPassManagerBuilderSetOptLevel(builder: PassManagerBuilderRef, opt_level: c_uint);
    pub fn LLVMAddStripDeadPrototypesPass(manager: PassManagerRef);
    pub fn LLVMAddStripSymbolsPass(manager: PassManagerRef);

    pub fn LLVMInitializeNVPTXTargetInfo();
    pub fn LLVMInitializeNVPTXTarget();
    pub fn LLVMInitializeNVPTXTargetMC();
    pub fn LLVMInitializeNVPTXAsmPrinter();
}
