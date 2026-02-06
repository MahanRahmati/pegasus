use thiserror::Error;

/// Application runtime errors.
///
/// Represents high-level errors that can occur during application workflows.
#[derive(Error, Debug)]
pub enum RuntimeError {
  #[error("Input Error: {0}")]
  Input(String),

  #[error("Refinement Error: {0}")]
  Refinement(String),
}

/// Result type for application runtime operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;
