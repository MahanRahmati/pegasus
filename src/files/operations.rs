use crate::files::errors::{FileError, FileResult};

/// Reads the entire contents of a file into a string.
///
/// # Arguments
///
/// * `file_path` - The path to the file to read
///
/// # Returns
///
/// A `FileResult<String>` containing the file contents or an error.
pub async fn read_to_string(file_path: &str) -> FileResult<String> {
  return tokio::fs::read_to_string(file_path)
    .await
    .map_err(|e| FileError::FileRead(e.to_string()));
}
