use clap::{arg, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
  /// Path to scan for node_modules folders
  #[arg(short, long, default_value_t = String::from("."))]
  pub path: String
}

