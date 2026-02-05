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

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Pegasus")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Pegasus v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,

  #[arg(short, long, conflicts_with = "file")]
  pub input: Option<String>,

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
  /// Reset configuration to default values
  ResetConfig,
}
