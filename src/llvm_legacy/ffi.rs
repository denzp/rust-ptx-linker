use cty::{c_char, c_uint};

use super::ffi_ty::*;
use super::message::*;

// Inspired by https://github.com/rust-lang/rust/blob/61452e506/src/librustc_llvm/ffi.rs
// Has some extra declarations compared to rustc FFI.

extern "C" {
    pub fn LLVMContextCreate() -> ContextRef;
    pub fn LLVMModuleCreateWithName(id: *const c_char) -> ModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(id: *const c_char, context: ContextRef) -> ModuleRef;

    pub fn LLVMGetTarget(module: ModuleRef) -> *const c_char;
    pub fn LLVMGetTargetFromTriple(
        triple: *const c_char,
        target: &mut TargetRef,
        out_message: &mut Message,
    ) -> c_uint;

    pub fn LLVMWriteBitcodeToFile(module: ModuleRef, path: *const c_char) -> c_uint;
    pub fn LLVMPrintModuleToFile(
        module: ModuleRef,
        path: *const c_char,
        out_message: &mut Message,
    ) -> Bool;

    pub fn LLVMTargetMachineEmitToFile(
        target_machine: TargetMachineRef,
        module: ModuleRef,
        path: *const c_char,
        codegen_type: CodeGenFileType,
        out_message: &mut Message,
    ) -> Bool;

    pub fn LLVMCreateTargetMachine(
        target: TargetRef,
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        level: CodeGenOptLevel,
        reloc: RelocMode,
        code_model: CodeModel,
    ) -> TargetMachineRef;

    pub fn LLVMCreateMemoryBufferWithMemoryRange(
        data: *const c_char,
        size: c_uint,
        buffer_name: *const c_char,
        requires_null_terminator: Bool,
    ) -> MemoryBufferRef;

    pub fn LLVMLinkModules2(dst: ModuleRef, src: ModuleRef) -> Bool;
    pub fn LLVMParseBitcodeInContext2(
        context: ContextRef,
        buffer: MemoryBufferRef,
        module: &mut ModuleRef,
    ) -> Bool;

    pub fn LLVMPassManagerBuilderCreate() -> PassManagerBuilderRef;
    pub fn LLVMPassManagerBuilderDispose(builder: PassManagerBuilderRef);
    pub fn LLVMPassManagerBuilderSetOptLevel(builder: PassManagerBuilderRef, opt_level: c_uint);

    pub fn LLVMCreatePassManager() -> PassManagerRef;
    pub fn LLVMDisposePassManager(manager: PassManagerRef);
    pub fn LLVMRunPassManager(manager: PassManagerRef, module: ModuleRef) -> Bool;

    pub fn LLVMSetModuleInlineAsm(module: ModuleRef, asm: *const c_char);
    pub fn LLVMAddGlobalDCEPass(manager: PassManagerRef);
    pub fn LLVMPassManagerBuilderPopulateLTOPassManager(
        builder: PassManagerBuilderRef,
        manager: PassManagerRef,
        internalize: Bool,
        run_inliner: Bool,
    );
    pub fn LLVMPassManagerBuilderPopulateModulePassManager(
        builder: PassManagerBuilderRef,
        manager: PassManagerRef,
    );

    pub fn LLVMInitializeNVPTXTargetInfo();
    pub fn LLVMInitializeNVPTXTarget();
    pub fn LLVMInitializeNVPTXTargetMC();
    pub fn LLVMInitializeNVPTXAsmPrinter();

    pub fn LLVMDisposeModule(module: ModuleRef);
    pub fn LLVMContextDispose(context: ContextRef);
    pub fn LLVMDisposeMessage(message: *const c_char);
    pub fn LLVMDisposeTargetMachine(target_machine: TargetMachineRef);
    pub fn LLVMDisposeMemoryBuffer(buffer: MemoryBufferRef);
}
