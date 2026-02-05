mod cli;
mod config;
mod files;
mod logging;

use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;
use crate::logging::set_verbose;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  set_verbose(cli.verbose);

  let config = match Config::load().await {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Configuration Error: {}", e);
      std::process::exit(1);
    }
  };

  vlog!("Hello, world!");
}
