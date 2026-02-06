use thiserror::Error;

/// File operation errors.
///
/// Represents errors that can occur during file and directory operations.
#[derive(Error, Debug)]
pub enum FileError {
  #[error(
    "Cannot read file '{0}'. Please check if the file exists and you have permission to access it."
  )]
  FileRead(String),
}

/// Result type for file operations.
pub type FileResult<T> = Result<T, FileError>;
