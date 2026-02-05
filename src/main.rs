mod cli;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  println!("Hello, world!");
}
