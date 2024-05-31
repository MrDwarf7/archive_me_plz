#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("Input cannot be empty")]
    EmptyInput,

    #[error("Only provided one argument - tool requires a min. of 3 arguments")]
    OnlyProvidedOneArgument,

    #[error("Only provided two arguments - tool requires a min. of 3 arguments")]
    OnlyProvideTwoArguments,

    #[error("Invalid input found")]
    InvalidInput,

    #[error("Failed to parse the command line integer argument")]
    ParsInt(#[from] std::num::ParseIntError),

    #[error("Failed to parse the command line date argument")]
    DateFormat(#[from] chrono::ParseError),

    #[error("Too many arguments provided")]
    TooManyArguments,

    #[error("Failed to read directory entries")]
    ReadDir,

    #[error("Failed to create directory")]
    CreateDir,

    #[error("Tokio join error while printing to std_out")]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error("Copied files and files to delete don't match, aborting")]
    CopiedFilesDontMatch,

    #[error("No files found in given directory")]
    NoFilesFoundInGivenDir,

    #[error("No files found in given directory or no files to move")]
    NoFilesOutsideOfGivenBounds,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Generic(e.to_string())
    }
}
