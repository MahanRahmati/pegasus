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

  /// Creates an LLM client configured with the current settings.
  ///
  /// # Returns
  ///
  /// A configured `LLMClient` instance.
  fn create_llm_client(&self) -> LLMClient {
    vlog!(
      "Initializing LLM client with model: {}",
      self.config.get_llm_model()
    );

    return LLMClient::new(
      self.config.get_llm_url(),
      self.config.get_llm_model(),
      self.config.get_llm_api_key(),
    );
  }

  /// Formats the refined text according to the specified output format.
  ///
  /// # Arguments
  ///
  /// * `refined_text` - The refined text to format
  /// * `format` - The desired output format
  ///
  /// # Returns
  ///
  /// A `RuntimeResult<String>` containing the formatted output or an error.
  fn format_output(
    &self,
    refined_text: String,
    format: OutputFormat,
  ) -> RuntimeResult<String> {
    return match format {
      OutputFormat::Text => Ok(refined_text),
      OutputFormat::Json => {
        let json_output = serde_json::json!({ "text": refined_text });
        serde_json::to_string(&json_output).map_err(|e| {
          RuntimeError::Refinement(format!("Failed to serialize JSON: {}", e))
        })
      }
    };
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

    let llm = self.create_llm_client();

    let refined_text = llm
      .refine_text(&input_text, &dictionary_words)
      .await
      .map_err(|e| RuntimeError::Refinement(e.to_string()))?;

    return self.format_output(refined_text, format);
  }

  /// Refines a Whisper JSON transcription using confidence scores.
  ///
  /// Parses the Whisper JSON, identifies low-confidence words,
  /// and sends the transcription to the LLM for refinement with
  /// confidence awareness to reduce hallucination.
  ///
  /// # Arguments
  ///
  /// * `input` - The inline text input of the Whisper JSON
  /// * `file_path` - The file path to the Whisper JSON file
  /// * `format` - The desired output format
  ///
  /// # Returns
  ///
  /// The refined text, or an error if refinement fails.
  pub async fn refine_whisper_transcription(
    &self,
    input: Option<String>,
    file_path: Option<String>,
    format: OutputFormat,
  ) -> RuntimeResult<String> {
    let input_text = InputReader::read_input(input, file_path)
      .await
      .map_err(|e| RuntimeError::Input(e.to_string()))?;

    let transcription: crate::input::transcription::WhisperTranscription =
      serde_json::from_str(&input_text).map_err(|e| {
        RuntimeError::Input(format!("Failed to parse Whisper JSON: {}", e))
      })?;

    let segment_count = transcription.segments.as_ref().map_or(0, |s| s.len());
    vlog!(
      "Loaded Whisper transcription: {} segments, {} words, duration: {:.1}s",
      segment_count,
      transcription.word_count(),
      transcription.duration_or_default()
    );

    let dictionary_words = self.load_dictionary().await?;
    let probability_threshold = self.config.get_whisper_probability_threshold();

    let llm = self.create_llm_client();

    let refined_text = llm
      .refine_whisper_transcription(
        &transcription,
        &dictionary_words,
        probability_threshold,
      )
      .await
      .map_err(|e| RuntimeError::Refinement(e.to_string()))?;

    return self.format_output(refined_text, format);
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
