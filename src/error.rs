pub enum NasError {
    FailedToReadFile,
    FailedToParse,
    Ignore,
    
}

impl From<std::io::Error> for NasError {
    fn from(value: std::io::Error) -> Self { Self::Ignore }
}

impl From<ron::error::SpannedError> for NasError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ignore
    }
}
