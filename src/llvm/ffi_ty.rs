use cty::c_uint;

// Inspired by https://github.com/rust-lang/rust/blob/61452e506/src/librustc_llvm/ffi.rs

#[allow(non_camel_case_types)]
pub enum MemoryBuffer_opaque {}
pub type MemoryBufferRef = *mut MemoryBuffer_opaque;

#[allow(non_camel_case_types)]
pub enum Module_opaque {}
pub type ModuleRef = *mut Module_opaque;

#[allow(non_camel_case_types)]
pub enum Context_opaque {}
pub type ContextRef = *mut Context_opaque;

#[allow(non_camel_case_types)]
pub enum TargetMachine_opaque {}
pub type TargetMachineRef = *mut TargetMachine_opaque;

#[allow(non_camel_case_types)]
pub enum Target_opaque {}
pub type TargetRef = *mut Target_opaque;

#[allow(non_camel_case_types)]
pub enum PassManagerBuilder_opaque {}
pub type PassManagerBuilderRef = *mut PassManagerBuilder_opaque;

#[allow(non_camel_case_types)]
pub enum PassManager_opaque {}
pub type PassManagerRef = *mut PassManager_opaque;

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum RelocMode {
    Default,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum CodeModel {
    Other,
    Default,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum CodeGenFileType {
    AssemblyFile,
    ObjectFile,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum CodeGenOptLevel {
    Other,
    None,
    Less,
    Default,
    Aggressive,
}

pub type Bool = c_uint;

#[allow(non_upper_case_globals)]
pub const True: Bool = 1;

#[allow(non_upper_case_globals)]
pub const False: Bool = 0;
