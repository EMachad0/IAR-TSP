use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatasetError {
    #[error("Missing argument on line {idx}: {text:?}")]
    MissingArgumentError { idx: usize, text: String },
    #[error("Too many arguments on line {idx}: {text:?}")]
    TooManyArgumentsError { idx: usize, text: String },
    #[error("Missing \"NODE_COORD_SECTION\" line")]
    MissingDataEntryTag,
    #[error("Missing \"EOF\" line")]
    MissingDataEndingTag,
}
