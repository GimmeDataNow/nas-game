#[allow(dead_code)]
#[derive(Debug)]
pub enum NasError {
    FailedToReadFile,
    FailedToParse,
    FailedToSerialize,
    FailedToWrite,
    FailedToCreateFolder,
    InvalidPath,
    Ignore,
    
}

impl From<std::io::Error> for NasError {
    fn from(_value: std::io::Error) -> Self { Self::Ignore }
}

impl From<ron::error::SpannedError> for NasError {
    fn from(_value: ron::error::SpannedError) -> Self { Self::Ignore }
}
