use serde::Deserialize;

/// OpenAI-compatible chat completion response.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
  pub choices: Vec<Choice>,
}

/// A choice in the chat completion response.
#[derive(Debug, Deserialize)]
pub struct Choice {
  pub message: ResponseMessage,
}

/// Message structure in the response.
#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
  pub content: String,
}
