/// Output format for refined text results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
  /// Plain text output
  Text,
  /// JSON output
  Json,
}

impl OutputFormat {
  /// Creates OutputFormat from CLI boolean flags.
  ///
  /// # Arguments
  ///
  /// * `output_json` - Whether to output JSON
  ///
  /// # Returns
  ///
  /// The appropriate `OutputFormat` variant.
  pub fn from_flags(output_json: bool) -> Self {
    if output_json {
      return Self::Json;
    }
    return Self::Text;
  }
}
