//! Whisper transcription data structures for structured refinement.
//!
//! This module provides types for parsing and working with Whisper JSON
//! transcription output, including word-level confidence scores and timestamps
//! to reduce hallucination during text refinement.
//!
//! ## Components
//!
//! - [`WhisperWord`]: Individual word with confidence and timing
//! - [`WhisperSegment`]: Segment of transcription with words
//! - [`WhisperTranscription`]: Complete transcription data

use serde::Deserialize;

/// Represents a single word in a Whisper transcription with timing and probability.
#[derive(Debug, Clone, Deserialize)]
pub struct WhisperWord {
  /// The word text (may include leading space)
  pub word: String,
  /// Probability score (0.0 to 1.0)
  pub probability: f64,
}

/// Represents a segment of transcribed speech.
#[derive(Debug, Clone, Deserialize)]
pub struct WhisperSegment {
  /// Segment text
  pub text: String,
  /// Individual words in this segment
  pub words: Vec<WhisperWord>,
}

/// Complete Whisper transcription data from JSON output.
///
/// Supports both full Whisper JSON (with word-level data) and simple
/// text-only formats. Optional fields default to None for simple formats.
#[derive(Debug, Clone, Deserialize)]
pub struct WhisperTranscription {
  /// Full text content
  pub text: Option<String>,
  /// Detected or specified language (optional for simple formats)
  pub language: Option<String>,
  /// Total duration in seconds (optional for simple formats)
  pub duration: Option<f64>,
  /// Segments of transcription with word-level data (optional)
  pub segments: Option<Vec<WhisperSegment>>,
}

impl WhisperTranscription {
  /// Returns all words with probability below the given threshold.
  ///
  /// Returns empty vector if no segments are present (simple format).
  ///
  /// # Arguments
  ///
  /// * `threshold` - The probability threshold (0.0 to 1.0)
  ///
  /// # Returns
  ///
  /// A vector of references to low-probability words.
  pub fn get_low_probability_words(&self, threshold: f64) -> Vec<&WhisperWord> {
    match &self.segments {
      None => return Vec::new(),
      Some(segments) => {
        return segments
          .iter()
          .flat_map(|segment| &segment.words)
          .filter(|word| word.probability < threshold)
          .collect();
      }
    }
  }

  /// Returns the number of words in the transcription.
  ///
  /// Returns 0 if no segments are present (simple format).
  ///
  /// # Returns
  ///
  /// The total word count.
  pub fn word_count(&self) -> usize {
    match &self.segments {
      None => return 0,
      Some(segments) => {
        return segments.iter().map(|segment| segment.words.len()).sum();
      }
    }
  }

  /// Returns the full text of the transcription.
  ///
  /// For simple formats, returns the text field directly.
  /// For full formats with segments, concatenates segment text.
  ///
  /// # Returns
  ///
  /// The transcription text, or empty string if none available.
  pub fn full_text(&self) -> String {
    // If we have a direct text field, use it
    if let Some(text) = &self.text {
      return text.clone();
    }

    // Otherwise, concatenate from segments
    match &self.segments {
      None => return String::new(),
      Some(segments) => {
        return segments
          .iter()
          .map(|s| s.text.as_str())
          .collect::<Vec<_>>()
          .join("\n");
      }
    }
  }

  /// Returns the language, or "unknown" if not specified.
  ///
  /// # Returns
  ///
  /// The detected language or "unknown".
  pub fn language_or_default(&self) -> String {
    return self
      .language
      .clone()
      .unwrap_or_else(|| "unknown".to_string());
  }

  /// Returns the duration, or 0.0 if not specified.
  ///
  /// # Returns
  ///
  /// The duration in seconds, or 0.0.
  pub fn duration_or_default(&self) -> f64 {
    return self.duration.unwrap_or(0.0);
  }
}
