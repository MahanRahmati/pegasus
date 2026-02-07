use crate::input::transcription::WhisperTranscription;

/// Builds the system prompt for text refinement.
///
/// Creates instructions for the LLM on how to refine transcription text,
/// including dictionary words to reduce hallucination.
///
/// # Arguments
///
/// * `dictionary_words` - List of words from the user's custom dictionary
///
/// # Returns
///
/// A system prompt string.
pub fn build_system_prompt(dictionary_words: &[String]) -> String {
  let dictionary_section = if dictionary_words.is_empty() {
    String::new()
  } else {
    format!(
      "\n\nUse the following dictionary terms correctly when they appear in the text:\n{}",
      dictionary_words.join(", ")
    )
  };

  return format!(
    "You are a helpful assistant that refines transcribed text. Your task is to:\n\
     1. Fix grammar, spelling, and punctuation errors\n\
     2. Preserve the original meaning and intent of the text\n\
     3. Maintain the original language\n\
     4. Do not add commentary or explanations\n\
     5. Only return the refined text, nothing else\n\
     6. Preserve paragraph breaks and basic formatting{}\n\n\
     Return only the refined text without any additional commentary or formatting.",
    dictionary_section
  );
}

/// Builds the user prompt with the input text.
///
/// # Arguments
///
/// * `input_text` - The transcription text to refine
///
/// # Returns
///
/// A user prompt string containing the input text.
pub fn build_user_prompt(input_text: &str) -> String {
  return format!(
    "Please refine the following transcribed text:\n\n{}",
    input_text
  );
}

/// Builds the system prompt for Whisper transcription refinement.
///
/// Creates instructions for the LLM on how to refine transcription text
/// with probability score awareness to reduce hallucination.
///
/// # Arguments
///
/// * `dictionary_words` - List of words from the user's custom dictionary
///
/// # Returns
///
/// A system prompt string.
pub fn build_whisper_system_prompt(dictionary_words: &[String]) -> String {
  let dictionary_section = if dictionary_words.is_empty() {
    String::new()
  } else {
    format!(
      "\n\nUse the following dictionary terms correctly when they appear in the text:\n{}",
      dictionary_words.join(", ")
    )
  };

  return format!(
    "You are a helpful assistant that refines transcribed text from speech recognition. \
     You have access to probability scores for each word. Your task is to:\n\
     1. Fix grammar, spelling, and punctuation errors\n\
     2. Preserve the original meaning and intent of the text\n\
     3. Maintain the original language\n\
     4. Pay special attention to low-probability words (flagged below) - verify them using context\n\
     5. Do not add commentary or explanations\n\
     6. Only return the refined text, nothing else\n\
     7. Preserve paragraph breaks and basic formatting{}\n\n\
     When you see low-probability words marked with [LOW PROBABILITY: X.XX], \
     carefully consider if they make sense in context. Use surrounding high-probability \
     words and overall meaning to determine the correct word.\n\n\
     Return only the refined text without any additional commentary or formatting.",
    dictionary_section
  );
}

/// Builds the user prompt with Whisper transcription data.
///
/// Formats the transcription with low-probability words flagged to help
/// the LLM make better decisions about ambiguous words.
///
/// For simple text-only formats without word-level data, falls back to
/// basic text refinement without probability flags.
///
/// # Arguments
///
/// * `transcription` - The Whisper transcription data
/// * `probability_threshold` - Words below this threshold will be flagged
///
/// # Returns
///
/// A user prompt string containing the formatted transcription.
pub fn build_whisper_user_prompt(
  transcription: &WhisperTranscription,
  probability_threshold: f64,
) -> String {
  // If we have segments with word-level data, use probability-aware formatting
  if let Some(segments) = &transcription.segments {
    let mut formatted_text = String::new();
    let low_probability_words =
      transcription.get_low_probability_words(probability_threshold);

    for segment in segments {
      let mut segment_text = segment.text.clone();

      for word in &low_probability_words {
        let trimmed_word = word.word.trim();
        if !trimmed_word.is_empty() {
          let flag = format!(
            "{} [LOW PROBABILITY: {:.2}]",
            trimmed_word, word.probability
          );
          segment_text = segment_text.replace(trimmed_word, &flag);
        }
      }

      formatted_text.push_str(&segment_text);
      formatted_text.push('\n');
    }

    return format!(
      "Please refine the following transcribed text ({}). \
       Words with probability scores below {:.2} are marked with [LOW PROBABILITY: X.XX]:\n\n{}",
      transcription.language_or_default(),
      probability_threshold,
      formatted_text
    );
  }

  // Simple format: no word-level data, just use the text directly
  let text = transcription.full_text();
  return format!(
    "Please refine the following transcribed text ({}):\n\n{}",
    transcription.language_or_default(),
    text
  );
}
