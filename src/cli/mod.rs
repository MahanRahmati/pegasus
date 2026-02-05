//! Command-line interface and argument parsing module.
//!
//! This module defines the CLI structure using `clap` for parsing command-line
//! arguments and subcommands. It provides type-safe argument handling and
//! automatic help generation.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Pegasus")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Pegasus v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,

  /// Use verbose output
  #[arg(short, long, default_value_t = false, global = true)]
  pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Reset configuration to default values
  ResetConfig,
}
