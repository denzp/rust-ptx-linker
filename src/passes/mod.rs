mod globals;
pub use self::globals::RenameGlobalsPass;

mod external_references;
pub use self::external_references::FindExternalReferencesPass;

mod internalize;
pub use self::internalize::{FindInternalFunctionsPass, FindInternalGlobalsPass};
