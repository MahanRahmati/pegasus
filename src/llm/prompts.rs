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
