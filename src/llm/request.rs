use serde::Serialize;

/// OpenAI-compatible chat completion request.
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
  model: String,
  messages: Vec<ChatMessage>,
}

impl ChatCompletionRequest {
  /// Creates a new `ChatCompletionRequest` with the specified model and messages.
  ///
  /// # Arguments
  ///
  /// * `model` - Model name to use (e.g., "llama3.2", "gpt-4")
  /// * `messages` - List of messages to send to the LLM
  ///
  /// # Returns
  ///
  /// A new `ChatCompletionRequest` instance.
  pub fn new(model: String, messages: Vec<ChatMessage>) -> Self {
    return ChatCompletionRequest { model, messages };
  }
}

/// OpenAI-compatible chat message structure.
#[derive(Debug, Serialize)]
pub struct ChatMessage {
  role: String,
  content: String,
}

impl ChatMessage {
  /// Creates a new `ChatMessage` with the specified role and content.
  ///
  /// # Arguments
  ///
  /// * `role` - Role of the message (e.g., "system", "user")
  /// * `content` - Content of the message
  ///
  /// # Returns
  ///
  /// A new `ChatMessage` instance.
  pub fn new(role: String, content: String) -> Self {
    return ChatMessage { role, content };
  }
}
