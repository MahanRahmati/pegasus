//! HTTP client module for network requests to external services.
//!
//! This module provides a simple HTTP client for communicating with remote
//! services. It supports JSON POST requests, and JSON response parsing.
//!
//! ## Main Components
//!
//! - [`HttpClient`]: HTTP client for making requests to external services
//! - [`NetworkError`]: Error types for network operations
//! - [`NetworkResult<T>`]: Result type alias for network operations
//!
//! ## Features
//!
//! - POST requests with JSON body and optional headers
//! - JSON response deserialization
//! - URL validation before requests

pub mod errors;

use std::collections::HashMap;

use serde::Serialize;

use crate::network::errors::{NetworkError, NetworkResult};
use crate::vlog;

/// HTTP client for network requests to external services.
///
/// Provides generic POST functionality with multipart form support.
#[derive(Debug, Clone)]
pub struct HttpClient {
  base_url: String,
}

impl HttpClient {
  /// Creates a new HttpClient with base URL.
  ///
  /// # Arguments
  ///
  /// * `base_url` - Base URL for all HTTP requests
  ///
  /// # Returns
  ///
  /// A new `HttpClient` instance.
  pub fn new(base_url: String) -> Self {
    return HttpClient { base_url };
  }

  /// Sends a POST request with JSON body to the given endpoint.
  ///
  /// Validates the service URL, sends the request with JSON body and optional
  /// headers, and deserializes the JSON response into the specified type.
  ///
  /// # Type Parameters
  ///
  /// * `T` - Type to deserialize the JSON response into
  /// * `B` - Type of the request body (must implement Serialize)
  ///
  /// # Arguments
  ///
  /// * `body` - JSON-serializable body to send in the request
  /// * `endpoint` - Endpoint path to append to the base URL
  /// * `headers` - Optional map of header names to values
  ///
  /// # Returns
  ///
  /// A `NetworkResult<T>` containing the deserialized response or an error.
  pub async fn post_with_json<T, B>(
    &self,
    body: &B,
    endpoint: &str,
    headers: Option<HashMap<String, String>>,
  ) -> NetworkResult<T>
  where
    T: serde::de::DeserializeOwned,
    B: Serialize,
  {
    self.check_url().await?;

    let client = reqwest::Client::new();

    let full_url = if self.base_url.ends_with("/") {
      format!("{}{}", self.base_url, endpoint)
    } else {
      format!("{}/{}", self.base_url, endpoint)
    };

    vlog!("Sending POST request to: {}", full_url);

    let mut request_builder = client.post(&full_url).json(body);

    if let Some(hdrs) = headers {
      for (key, value) in hdrs {
        request_builder = request_builder.header(key, value);
      }
    }

    let response = request_builder
      .send()
      .await
      .map_err(|_| NetworkError::RequestFailed)?;

    vlog!(
      "Received response from service. Status: {}",
      response.status()
    );

    if !response.status().is_success() {
      return Err(NetworkError::ResponseError);
    }

    let parsed_response = response
      .json::<T>()
      .await
      .map_err(|_| NetworkError::DecodeError)?;

    return Ok(parsed_response);
  }

  async fn check_url(&self) -> NetworkResult<()> {
    vlog!("Checking if service URL is reachable...");

    let _url = reqwest::Url::parse(&self.base_url).map_err(|e| {
      vlog!("Invalid URL format: {}", e);
      NetworkError::InvalidURL(self.base_url.clone())
    })?;

    let client = reqwest::Client::new();

    let response = client.get(&self.base_url).send().await.map_err(|e| {
      vlog!("Failed to connect to URL: {}", e);
      NetworkError::RequestFailed
    })?;

    let status = response.status();
    if status != reqwest::StatusCode::OK
      && status != reqwest::StatusCode::NOT_FOUND
    {
      vlog!("URL returned unexpected status: {}", status);
      return Err(NetworkError::InvalidURL(self.base_url.clone()));
    }

    vlog!("Service URL is reachable with status: {}", status);

    return Ok(());
  }
}
