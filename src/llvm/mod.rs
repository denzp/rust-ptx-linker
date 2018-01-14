use cty::c_uint;

mod message;
pub use self::message::*;

mod ffi_ty;
pub use self::ffi_ty::*;

mod ffi;
pub use self::ffi::*;

extern "C" {
    /// Returns count of external references that are found.
    /// Also writes semicolon (";") separated list to the `out_messages`.
    ///
    /// Defined in `llvm/find-external-refs.cpp`
    pub fn FindExternalReferences(module: ModuleRef, out_message: &mut Message) -> c_uint;

    // Remove every function but kernels and their dependent functions.
    ///
    /// Defined in `llvm/internalize.cpp`
    pub fn StripInternalFunctions(module: ModuleRef);

    // Rename Global Variables to make them PTX-friendly.
    ///
    /// Defined in `llvm/rename-globals.cpp`
    pub fn RenameGlobalVariables(module: ModuleRef);
}
