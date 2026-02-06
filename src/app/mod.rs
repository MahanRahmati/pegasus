//! Application orchestration module for Pegasus.
//!
//! ## Main Components
//!
//! - [`App`]: The primary application orchestrator that manages all workflows
//! - [`RuntimeError`]: Error types for application-level failures
//! - [`RuntimeResult<T>`]: Result type alias for application operations

pub mod errors;

use crate::app::errors::{RuntimeError, RuntimeResult};
use crate::config::Config;
use crate::files::operations;
use crate::input::InputReader;
use crate::llm::client::LLMClient;
use crate::output::format::OutputFormat;
use crate::vlog;

/// Main application orchestrator for Pegasus.
///
/// Coordinates text refinement operations using the provided configuration settings.
pub struct App {
  config: Config,
}

impl App {
  /// Creates a new App instance with the given configuration.
  ///
  /// # Arguments
  ///
  /// * `config` - Configuration containing all application settings
  ///
  /// # Returns
  ///
  /// A new `App` instance.
  pub fn new(config: Config) -> Self {
    return App { config };
  }

  /// Refines the input text using the LLM.
  ///
  /// # Arguments
  ///
  /// * `input` - The inline text input
  /// * `file_path` - The file path for input text
  /// * `format` - The desired output format
  ///
  /// # Returns
  ///
  /// The refined text, or an error if refinement fails.
  pub async fn refine_text(
    &self,
    input: Option<String>,
    file_path: Option<String>,
    format: OutputFormat,
  ) -> RuntimeResult<String> {
    let input_text = InputReader::read_input(input, file_path)
      .await
      .map_err(|e| RuntimeError::Input(e.to_string()))?;

    let dictionary_words = self.load_dictionary().await?;

    vlog!(
      "Initializing LLM client with model: {}",
      self.config.get_llm_model()
    );

    let llm = LLMClient::new(
      self.config.get_llm_url(),
      self.config.get_llm_model(),
      self.config.get_llm_api_key(),
    );

    let refined_text = llm
      .refine_text(&input_text, &dictionary_words)
      .await
      .map_err(|e| RuntimeError::Refinement(e.to_string()))?;

    let output = match format {
      OutputFormat::Text => refined_text,
      OutputFormat::Json => {
        let json_output = serde_json::json!({ "text": refined_text });
        serde_json::to_string(&json_output).map_err(|e| {
          RuntimeError::Refinement(format!("Failed to serialize JSON: {}", e))
        })?
      }
    };

    return Ok(output);
  }

  /// Loads dictionary words from the configured dictionary file.
  ///
  /// Reads the dictionary file and returns a list of words, one per line.
  /// Skips empty lines and lines starting with '#' (comments).
  ///
  /// # Returns
  ///
  /// A `RuntimeResult<Vec<String>>` containing the dictionary words or an error.
  async fn load_dictionary(&self) -> RuntimeResult<Vec<String>> {
    let dictionary_path = self.config.get_custom_dictionary_path();

    if dictionary_path.is_empty() {
      vlog!("No custom dictionary configured");
      return Ok(Vec::new());
    }

    vlog!("Loading dictionary from: {}", dictionary_path);

    let content =
      operations::read_to_string(&dictionary_path)
        .await
        .map_err(|e| {
          RuntimeError::Input(format!("Failed to read dictionary: {}", e))
        })?;

    let words: Vec<String> = content
      .lines()
      .map(|line| line.trim())
      .filter(|line| !line.is_empty() && !line.starts_with('#'))
      .map(|line| line.to_string())
      .collect();

    vlog!("Loaded {} dictionary words", words.len());

    return Ok(words);
  }
}
