use failure::Fail;

#[derive(Debug, Fail)]
pub enum LinkerError {
    #[fail(display = "No output path is specified")]
    NoOutputPathError,

    #[fail(display = "Expected path, got `{}`", _0)]
    PathArgumentError(String),

    #[fail(display = "Undefined references: {:?}", _0)]
    UndefinedReferences(Vec<String>),
}
