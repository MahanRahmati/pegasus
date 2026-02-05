mod cli;
mod files;
mod logging;

use clap::Parser;

use crate::cli::Cli;
use crate::logging::set_verbose;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  set_verbose(cli.verbose);

  vlog!("Hello, world!");
}
