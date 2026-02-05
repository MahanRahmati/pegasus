use thiserror::Error;

/// Input reading errors.
///
/// Represents errors that can occur when reading input from various sources.
#[derive(Error, Debug)]
pub enum InputError {
  #[error("Failed to read file '{path}': {error}")]
  FileReadError { path: String, error: String },

  #[error("Input is empty")]
  EmptyInput,

  #[error("No input provided: use --file or --text")]
  NoInputProvided,
}

/// Result type for input reading operations.
pub type InputResult<T> = Result<T, InputError>;
