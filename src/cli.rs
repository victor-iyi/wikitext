use clap::{Parser, ValueEnum};
use std::{fmt, path::PathBuf, str::FromStr};

/// Command Linea argument parser for application.
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

  /// List of possible tokenizer algorithms to use.
  #[arg(
    long, value_enum, default_value_t=TokenizerType::Bpe,
  )]
  pub tokenizer: TokenizerType,

  /// List of possible pre-tokenizer rules to use.
  #[arg(long, value_enum, default_value_t=PreTokenizerType::ByteLevel)]
  pub pre_tokenizer: PreTokenizerType,

  /// Train the tokenizer
  #[arg(long)]
  pub train: bool,

  /// Prettify trainer output.
  #[arg(long)]
  pub pretty: bool,
}

/// Core algorithms used to actually tokenize the text.
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum TokenizerType {
  /// One of the most popular subword tokenization algorithm. The Byte-Pair-Encoding
  /// works by statring with characters, while merging those that are the most
  /// frequently seen together, thus creating new tokens.
  Bpe,

  /// Unigram is also a subword tokenization algorithm, and works by trying to
  /// identify the best set of subword tokens to maximize the probability for a
  /// given sentence.
  Unigram,

  /// This is the "classic" tokenization algorithm. It simply map words to IDs
  /// without anything fancy.
  WordLevel,

  /// This is a subword tokenization algorithm quite similar to BPE, used mainly
  /// by Google in models like BERT. It uses a greedy algorithm, that tries to
  /// build long words first, splitting in mujltiple tokens when entire words
  /// don't exit in the vocabulary.
  WordPiece,
}

impl fmt::Display for TokenizerType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::Bpe => "bpe",
      Self::Unigram => "unigram",
      Self::WordLevel => "word-level",
      Self::WordPiece => "word-peice",
    };
    s.fmt(f)
  }
}

impl FromStr for TokenizerType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "bpe" => Ok(Self::Bpe),
      "unigram" => Ok(Self::Unigram),
      "word-level" => Ok(Self::WordLevel),
      "word-peice" => Ok(Self::WordPiece),
      _ => Err(format!("Unknown tokenizer {s}")),
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum PreTokenizerType {
  /// Splits on whitespace while remapping all the bytes to a set of visible
  /// characters.
  ByteLevel,

  /// Splits on word boundaries (using regular expression: \w+|[^\w\s]+
  Whitespace,
}

impl fmt::Display for PreTokenizerType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::ByteLevel => "byte-level",
      Self::Whitespace => "whitespace",
    };
    s.fmt(f)
  }
}

impl FromStr for PreTokenizerType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "byte-level" => Ok(Self::ByteLevel),
      "whitespace" => Ok(Self::Whitespace),
      _ => Err(format!("Unknown pre-tokenizer {s}")),
    }
  }
}
