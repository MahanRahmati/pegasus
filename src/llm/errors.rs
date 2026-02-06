use thiserror::Error;

/// LLM-related errors.
///
/// Represents errors that can occur during LLM API communication and text refinement.
#[derive(Error, Debug)]
pub enum LLMError {
  #[error("LLM API request failed: {0}")]
  ApiRequestFailed(String),

  #[error("Invalid API response: {0}")]
  InvalidResponse(String),

  #[error("Text refinement failed: {0}")]
  RefinementFailed(String),
}

/// Result type for LLM operations.
pub type LLMResult<T> = Result<T, LLMError>;
