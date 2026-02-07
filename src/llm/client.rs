use std::collections::HashMap;

use crate::input::transcription::WhisperTranscription;
use crate::llm::errors::{LLMError, LLMResult};
use crate::llm::prompts::{
  build_system_prompt, build_user_prompt, build_whisper_system_prompt,
  build_whisper_user_prompt,
};
use crate::llm::request::{ChatCompletionRequest, ChatMessage};
use crate::llm::response::ChatCompletionResponse;
use crate::network::HttpClient;
use crate::vlog;

/// LLM client for text refinement using OpenAI-compatible APIs.
///
/// Provides methods to refine transcribed text using local or remote
/// LLM services that support the OpenAI chat completions API format.
#[derive(Debug, Clone)]
pub struct LLMClient {
  base_url: String,
  model: String,
  api_key: String,
}

impl LLMClient {
  /// Creates a new LLMClient with the given configuration.
  ///
  /// # Arguments
  ///
  /// * `base_url` - Base URL for the LLM API
  /// * `model` - Model name to use
  /// * `api_key` - Optional API key for authenticated endpoints
  ///
  /// # Returns
  ///
  /// A new `LLMClient` instance.
  pub fn new(base_url: String, model: String, api_key: String) -> Self {
    return LLMClient {
      base_url,
      model,
      api_key,
    };
  }

  /// Executes the LLM refinement request with given prompts.
  ///
  /// # Arguments
  ///
  /// * `system_prompt` - The system prompt for the LLM
  /// * `user_prompt` - The user prompt containing text to refine
  ///
  /// # Returns
  ///
  /// A `LLMResult<String>` containing the refined text or an error.
  async fn execute_refinement(
    &self,
    system_prompt: String,
    user_prompt: String,
  ) -> LLMResult<String> {
    let request = ChatCompletionRequest::new(
      self.model.clone(),
      vec![
        ChatMessage::new("system".to_string(), system_prompt),
        ChatMessage::new("user".to_string(), user_prompt),
      ],
    );

    let mut headers: HashMap<String, String> = HashMap::new();

    if !self.api_key.is_empty() {
      headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", self.api_key),
      );
      vlog!("Using API key authentication");
    }

    let headers_opt = if headers.is_empty() {
      None
    } else {
      Some(headers)
    };

    let http_client = HttpClient::new(self.base_url.clone());

    let completion: ChatCompletionResponse = http_client
      .post_with_json(&request, "v1/chat/completions", headers_opt)
      .await
      .map_err(|e| LLMError::ApiRequestFailed(e.to_string()))?;

    let refined_text = completion
      .choices
      .first()
      .ok_or_else(|| {
        LLMError::InvalidResponse("No choices in response".to_string())
      })?
      .message
      .content
      .trim()
      .to_string();

    if refined_text.is_empty() {
      return Err(LLMError::RefinementFailed(
        "LLM returned empty content".to_string(),
      ));
    }

    return Ok(refined_text);
  }

  /// Refines the input text using the LLM.
  ///
  /// Sends the text to the LLM with appropriate system and user prompts,
  /// including dictionary words to reduce hallucination.
  ///
  /// # Arguments
  ///
  /// * `input_text` - The transcription text to refine
  /// * `dictionary_words` - List of words from the user's custom dictionary
  ///
  /// # Returns
  ///
  /// A `LLMResult<String>` containing the refined text or an error.
  pub async fn refine_text(
    &self,
    input_text: &str,
    dictionary_words: &[String],
  ) -> LLMResult<String> {
    vlog!("Preparing LLM request for text refinement");

    let system_prompt = build_system_prompt(dictionary_words);
    let user_prompt = build_user_prompt(input_text);

    let refined_text =
      self.execute_refinement(system_prompt, user_prompt).await?;

    vlog!("Text refinement completed successfully");

    return Ok(refined_text);
  }

  /// Refines Whisper transcription using confidence scores to reduce hallucination.
  ///
  /// Sends the transcription to the LLM with low-confidence words flagged,
  /// allowing the LLM to make better decisions about ambiguous words.
  ///
  /// # Arguments
  ///
  /// * `transcription` - The Whisper transcription data with confidence scores
  /// * `dictionary_words` - List of words from the user's custom dictionary
  /// * `probability_threshold` - Words below this threshold will be flagged
  ///
  /// # Returns
  ///
  /// A `LLMResult<String>` containing the refined text or an error.
  pub async fn refine_whisper_transcription(
    &self,
    transcription: &WhisperTranscription,
    dictionary_words: &[String],
    probability_threshold: f64,
  ) -> LLMResult<String> {
    vlog!("Preparing LLM request for Whisper transcription refinement");
    vlog!(
      "Low probability threshold: {}, words flagged: {}",
      probability_threshold,
      transcription
        .get_low_probability_words(probability_threshold)
        .len()
    );

    let system_prompt = build_whisper_system_prompt(dictionary_words);
    let user_prompt =
      build_whisper_user_prompt(transcription, probability_threshold);

    let refined_text =
      self.execute_refinement(system_prompt, user_prompt).await?;

    vlog!("Whisper transcription refinement completed successfully");

    return Ok(refined_text);
  }
}
