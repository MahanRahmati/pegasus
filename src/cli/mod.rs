//! Command-line interface and argument parsing module.
//!
//! This module defines the CLI structure using `clap` for parsing command-line
//! arguments and subcommands. It provides type-safe argument handling and
//! automatic help generation.
//!
//! ## Commands
//!
//! - `--input <text>`: Refine the input text
//! - `--file <path>`: Refine the input text from a file
//! - `reset-config`: Reset configuration to default values
//! - `whisper-transcribe --input <json>`: Refine using Whisper JSON transcription with confidence scores from the input text.
//! - `whisper-transcribe --file <path>`: Refine using Whisper JSON transcription with confidence scores from a file

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Pegasus")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Pegasus v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,

  /// Input text to refine
  #[arg(short, long, conflicts_with = "file")]
  pub input: Option<String>,

  /// Path to the text file to refine
  #[arg(short, long, conflicts_with = "input")]
  pub file: Option<String>,

  /// Use verbose output
  #[arg(short, long, default_value_t = false, global = true)]
  pub verbose: bool,

  /// Output result in JSON format
  #[arg(short = 'j', long, default_value_t = false)]
  pub output_json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
  WhisperTranscribe {
    /// Input text from Whisper JSON transcription to refine
    #[arg(short, long, conflicts_with = "file")]
    input: Option<String>,

    /// Path to the Whisper JSON transcription file to refine
    #[arg(short, long, conflicts_with = "input")]
    file: Option<String>,

    /// Output result in JSON format
    #[arg(short = 'j', long, default_value_t = false)]
    output_json: bool,
  },

  /// Reset configuration to default values
  ResetConfig,
}
