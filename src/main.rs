use clap::Parser;
use wikitext::Cli;

fn main() {
  let args = Cli::parse();
  println!("{args:?}");
}
