mod rename;
pub use self::rename::{RenameFunctionsPass, RenameGlobalsPass};

mod external_references;
pub use self::external_references::FindExternalReferencesPass;

mod internalize;
pub use self::internalize::InternalizePass;
