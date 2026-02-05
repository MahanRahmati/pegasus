mod app;
mod cli;
mod config;
mod files;
mod logging;

use clap::Parser;

use crate::app::App;
use crate::cli::{Cli, Commands};
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

  let app = App::new(config);
  let input_text = match cli.input {
    Some(input) => input,
    None => match cli.file {
      Some(file) => {
        match files::operations::read_to_string(file.as_str()).await {
          Ok(content) => content,
          Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
          }
        }
      }
      None => {
        eprintln!("Please provide input text or a file path.");
        std::process::exit(1);
      }
    },
  };

  let result = match cli.command {
    Some(Commands::ResetConfig) => match Config::reset_to_defaults().await {
      Ok(_) => {
        println!("Configuration has been reset to default values.");
        return;
      }
      Err(e) => {
        eprintln!("Failed to reset configuration: {}", e);
        std::process::exit(1);
      }
    },
    None => app.refine_text(input_text).await,
  };

  match result {
    Ok(output) => println!("{}", output),
    Err(e) => {
      eprintln!("{}", e);
      std::process::exit(1);
    }
  }
}
