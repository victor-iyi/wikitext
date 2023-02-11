use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
  /// Path to the train, test & valid data.
  #[arg(
    short, long, value_hint=clap::ValueHint::DirPath,
    default_value = "data/wikitext-103-raw"
  )]
  pub data_dir: PathBuf,

  /// Path to save trained tokenizer.
  #[arg(short, long, default_value = "data/wikitext-tokenizer.json")]
  pub save_path: PathBuf,

  /// List of possible tokenizers.
  #[arg(
    long, value_enum, default_value_t=TokenizerType::BPE,
  )]
  pub tokenizer: TokenizerType,

  /// Train the tokenizer
  #[arg(long)]
  pub train: bool,

  /// Prettify trainer output.
  #[arg(long)]
  pub pretty: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum TokenizerType {
  /// Byte-Pair Encoding tokenizer
  BPE,

  /// WordPiece toeknizer
  WordPiece,
}

impl std::fmt::Display for TokenizerType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::BPE => "bpe",
      Self::WordPiece => "word-peice",
    };
    s.fmt(f)
  }
}

impl std::str::FromStr for TokenizerType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "bpe" => Ok(Self::BPE),
      "word-peice" => Ok(Self::WordPiece),
      _ => Err(format!("Unknown tokenizer {s}")),
    }
  }
}
