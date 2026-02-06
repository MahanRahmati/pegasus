//! LLM module for text refinement.
//!
//! This module provides integration with LLM services using OpenAI-compatible
//! APIs for refining transcribed text.
//!
//! ## Main Components
//!
//! - [`LLMClient`]: HTTP client for LLM API communication
//! - [`LLMError`]: Error types for LLM operations
//! - [`LLMResult<T>`]: Result type alias for LLM operations

pub mod client;
pub mod errors;
pub mod prompts;
mod request;
mod response;
