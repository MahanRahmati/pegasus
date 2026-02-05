//! Input reading module for reading input from various sources.
//!
//! This module provides utilities for reading input from various sources
//! including input and files.

pub mod errors;

use crate::files::operations;
use crate::input::errors::{InputError, InputResult};

/// Input source enumeration.
#[derive(Debug, Clone)]
enum InputSource {
  /// Inline text input.
  Input(String),
  /// Input from a file.
  File(String),
}

impl InputSource {
  /// Resolves the input source based on the provided input and file path.
  ///
  /// # Arguments
  ///
  /// * `input` - The inline text input
  /// * `file_path` - The file path for input text
  ///
  /// # Returns
  ///
  /// Returns the input source, or an error if no input is provided.
  fn resolve_input_source(
    input: Option<String>,
    file_path: Option<String>,
  ) -> InputResult<InputSource> {
    if let Some(input) = input {
      return Ok(InputSource::Input(input));
    }

    if let Some(file_path) = file_path {
      return Ok(InputSource::File(file_path));
    }

    return Err(InputError::NoInputProvided);
  }

  /// Reads input from the resolved input source.
  ///
  /// # Returns
  ///
  /// Returns the input text, or an error if input reading fails.
  pub async fn read_from_input_source(&self) -> InputResult<String> {
    return match self {
      InputSource::Input(input) => {
        if input.trim().is_empty() {
          return Err(InputError::EmptyInput);
        }
        return Ok(input.clone());
      }
      InputSource::File(file) => {
        let content =
          operations::read_to_string(file.as_str())
            .await
            .map_err(|e| InputError::FileReadError {
              path: file.to_string(),
              error: e.to_string(),
            })?;
        if content.trim().is_empty() {
          return Err(InputError::EmptyInput);
        }
        return Ok(content);
      }
    };
  }
}

pub struct InputReader {}

impl InputReader {
  /// Reads input from the provided input and file path.
  ///
  /// # Arguments
  ///
  /// * `input` - The inline text input
  /// * `file_path` - The file path for input text
  ///
  /// # Returns
  ///
  /// Returns the input text, or an error if input reading fails.
  pub async fn read_input(
    input: Option<String>,
    file_path: Option<String>,
  ) -> InputResult<String> {
    let input_source = InputSource::resolve_input_source(input, file_path)?;
    let input_text = input_source.read_from_input_source().await?;
    return Ok(input_text);
  }
}
