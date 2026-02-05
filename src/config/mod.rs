//! Configuration management module with XDG compliance.
//!
//! This module handles loading, parsing, and accessing application configuration
//! from TOML files stored in XDG-compliant directories. It provides default values
//! for all settings and supports configuration reset operations.
//!
//! ## Configuration Sections
//!
//! - [`LLMConfig`]: LLM service settings
//! - [`GeneralConfig`]: General application behavior settings
//!
//! ## Configuration File Location
//!
//! Configuration is loaded from:
//! - `$XDG_CONFIG_HOME/pegasus/config.toml`
//! - Falls back to defaults if no config file exists

pub mod errors;

use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::config::errors::{ConfigError, ConfigResult};
use crate::files::operations;

const DEFAULT_DIRECTORY: &str = "pegasus";
const DEFAULT_CONFIG_NAME: &str = "config.toml";
const DEFAULT_LLM_URL: &str = "http://127.0.0.1:8080";

/// Main configuration structure for the Pegasus application.
///
/// This struct contains all configuration sections including LLM settings,
/// and general application preferences.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub llm: LLMConfig,
  pub general: GeneralConfig,
}

/// Configuration for the LLM service.
///
/// Contains settings for the LLM API endpoint.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LLMConfig {
  pub url: Option<String>,
}

/// General application configuration.
///
/// Contains settings that affect overall application behavior.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GeneralConfig {
  pub custom_dictionary_path: Option<String>,
}

impl Config {
  /// Loads configuration from XDG-compliant config directory.
  ///
  /// Attempts to read and parse the configuration file from the standard
  /// XDG config location. If no config file exists, returns default configuration.
  ///
  /// # Returns
  ///
  /// A `ConfigResult<Config>` containing the loaded configuration or an error.
  pub async fn load() -> ConfigResult<Config> {
    let xdg_dirs = BaseDirectories::with_prefix(DEFAULT_DIRECTORY);
    let config_path = match xdg_dirs.find_config_file(DEFAULT_CONFIG_NAME) {
      Some(path) => path,
      None => {
        let default_config = Config::default();
        return Ok(default_config);
      }
    };
    return Config::load_from_path(config_path).await;
  }

  /// Gets the LLM URL.
  ///
  /// Returns the configured URL or the default localhost URL if not set.
  ///
  /// # Returns
  ///
  /// A `String` containing the LLM URL.
  pub fn get_llm_url(&self) -> String {
    return self
      .llm
      .url
      .clone()
      .unwrap_or(String::from(DEFAULT_LLM_URL));
  }

  /// Gets the custom dictionary path.
  ///
  /// Returns the configured custom dictionary path or an empty string if not set.
  ///
  /// # Returns
  ///
  /// A `String` containing the custom dictionary path.
  pub fn get_custom_dictionary_path(&self) -> String {
    return self
      .general
      .custom_dictionary_path
      .clone()
      .unwrap_or(String::new());
  }

  /// Resets the configuration to default values and saves it.
  ///
  /// Creates a new default configuration and saves it to the XDG config directory,
  /// overwriting any existing configuration file.
  ///
  /// # Returns
  ///
  /// A `ConfigResult<()>` indicating success or failure.
  pub async fn reset_to_defaults() -> ConfigResult<()> {
    let default_config = Config::default();
    let xdg_dirs = BaseDirectories::with_prefix(DEFAULT_DIRECTORY);
    let config_path = xdg_dirs
      .place_config_file(DEFAULT_CONFIG_NAME)
      .map_err(|e| ConfigError::FileRead(e.to_string()))?;
    return Config::save_to_path(default_config, config_path).await;
  }

  /// Loads configuration from a specific file path.
  ///
  /// This method is intended for testing purposes to allow loading
  /// configuration from temporary directories instead of the user's
  /// real config directory.
  ///
  /// # Arguments
  ///
  /// * `config_path` - Path to the configuration file to load
  ///
  /// # Returns
  ///
  /// A `ConfigResult<Config>` containing the loaded configuration or an error.
  pub(crate) async fn load_from_path(
    config_path: PathBuf,
  ) -> ConfigResult<Config> {
    let config_content =
      operations::read_to_string(&config_path.to_string_lossy())
        .await
        .map_err(|e| ConfigError::FileRead(e.to_string()))?;
    let config = toml::from_str(&config_content)
      .map_err(|e| ConfigError::Parse(e.to_string()))?;
    return Ok(config);
  }

  /// Saves configuration to a specific file path.
  ///
  /// This method is intended for testing purposes to allow saving
  /// configuration to temporary directories instead of the user's
  /// real config directory.
  ///
  /// # Arguments
  ///
  /// * `config` - The configuration to save
  /// * `config_path` - Path where the configuration should be saved
  ///
  /// # Returns
  ///
  /// A `ConfigResult<()>` indicating success or failure.
  pub(crate) async fn save_to_path(
    config: Config,
    config_path: PathBuf,
  ) -> ConfigResult<()> {
    let config_content = toml::to_string_pretty(&config)
      .map_err(|e| ConfigError::Parse(e.to_string()))?;
    tokio::fs::write(&config_path, config_content)
      .await
      .map_err(|e| ConfigError::FileRead(e.to_string()))?;
    return Ok(());
  }

  /// Resets configuration to defaults at a specific path.
  ///
  /// This method is intended for testing purposes to reset configuration
  /// in temporary directories instead of the user's real config directory.
  ///
  /// # Arguments
  ///
  /// * `config_path` - Path where the default configuration should be saved
  ///
  /// # Returns
  ///
  /// A `ConfigResult<()>` indicating success or failure.
  #[cfg(test)]
  pub(crate) async fn reset_to_defaults_at_path(
    config_path: PathBuf,
  ) -> ConfigResult<()> {
    let default_config = Config::default();
    return Config::save_to_path(default_config, config_path).await;
  }
}

impl Default for Config {
  fn default() -> Self {
    return Config {
      llm: LLMConfig {
        url: Some(String::from(DEFAULT_LLM_URL)),
      },
      general: GeneralConfig {
        custom_dictionary_path: Some(String::new()),
      },
    };
  }
}
