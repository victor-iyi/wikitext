use tokenizers::{
  models::{
    bpe::{BpeTrainerBuilder, BPE},
    unigram::{Unigram, UnigramTrainerBuilder},
    wordlevel::{WordLevel, WordLevelTrainerBuilder},
    wordpiece::{WordPiece, WordPieceTrainerBuilder},
  },
  normalizers::{strip::Strip, unicode::NFC, utils::Sequence},
  pre_tokenizers::byte_level::ByteLevel,
  AddedToken, Result, TokenizerBuilder,
};

use crate::{cli::TokenizerType, Cli};

/// Train tokenizer given model type.
pub fn train(args: &super::Cli) -> Result<()> {
  // training, testing & validation files.
  let files: Vec<String> = std::fs::read_dir(&args.data_dir)
    .unwrap()
    .map(|f| f.unwrap().path().to_str().unwrap().to_string())
    .collect();

  let special_tokens = vec![
    AddedToken::from(String::from("[START]"), true),
    AddedToken::from(String::from("[END]"), true),
    AddedToken::from(String::from("[PAD]"), true),
    AddedToken::from(String::from("[MASK]"), true),
    AddedToken::from(String::from("[UNK]"), true),
  ];

  match args.tokenizer {
    TokenizerType::Bpe => train_bpe(args, &files, &special_tokens),
    TokenizerType::Unigram => train_unigram(args, &files, &special_tokens),
    TokenizerType::WordLevel => train_word_level(args, &files, &special_tokens),
    TokenizerType::WordPiece => train_word_piece(args, &files, &special_tokens),
  }
}

/// Train a Byte-Pair Encoding tokenizer.
pub fn train_bpe(
  args: &Cli,
  files: &[String],
  special_tokens: &[AddedToken],
) -> Result<()> {
  let mut tokenizer = TokenizerBuilder::new()
    .with_model(BPE::default())
    .with_normalizer(Some(Sequence::new(vec![
      Strip::new(true, true).into(),
      NFC.into(),
    ])))
    .with_pre_tokenizer(Some(ByteLevel::default()))
    .with_post_processor(Some(ByteLevel::default()))
    .with_decoder(Some(ByteLevel::default()))
    .build()?;

  let mut trainer = BpeTrainerBuilder::new()
    .show_progress(true)
    .special_tokens(special_tokens.to_owned())
    .build();

  // train tokenizer on files.
  tokenizer
    .train_from_files(&mut trainer, files.to_owned())?
    .save(&args.save_path, args.pretty)?;

  Ok(())
}

/// Train a `WordPiece` tokenizer.
pub fn train_word_piece(
  args: &Cli,
  files: &[String],
  special_tokens: &[AddedToken],
) -> Result<()> {
  let mut tokenizer = TokenizerBuilder::new()
    .with_model(WordPiece::default())
    .with_normalizer(Some(Sequence::new(vec![
      Strip::new(true, true).into(),
      NFC.into(),
    ])))
    .with_pre_tokenizer(Some(ByteLevel::default()))
    .with_post_processor(Some(ByteLevel::default()))
    .with_decoder(Some(ByteLevel::default()))
    .build()?;

  let mut trainer = WordPieceTrainerBuilder::new()
    .show_progress(true)
    .special_tokens(special_tokens.to_owned())
    .build();

  // train tokenizer on files.
  tokenizer
    .train_from_files(&mut trainer, files.to_owned())?
    .save(&args.save_path, args.pretty)?;

  Ok(())
}

/// Train a `Unigram` tokenizer.
pub fn train_unigram(
  args: &Cli,
  files: &[String],
  special_tokens: &[AddedToken],
) -> Result<()> {
  let mut tokenizer = TokenizerBuilder::new()
    .with_model(Unigram::default())
    .with_normalizer(Some(Sequence::new(vec![
      Strip::new(true, true).into(),
      NFC.into(),
    ])))
    .with_pre_tokenizer(Some(ByteLevel::default()))
    .with_post_processor(Some(ByteLevel::default()))
    .with_decoder(Some(ByteLevel::default()))
    .build()?;

  let mut trainer = UnigramTrainerBuilder::default()
    .show_progress(true)
    .special_tokens(special_tokens.to_owned())
    .build()?;

  // train tokenizer on files.
  tokenizer
    .train_from_files(&mut trainer, files.to_owned())?
    .save(&args.save_path, args.pretty)?;
  Ok(())
}

/// Train a `WordLevel` tokenizer.
pub fn train_word_level(
  args: &Cli,
  files: &[String],
  special_tokens: &[AddedToken],
) -> Result<()> {
  let mut tokenizer = TokenizerBuilder::new()
    .with_model(WordLevel::default())
    .with_normalizer(Some(Sequence::new(vec![
      Strip::new(true, true).into(),
      NFC.into(),
    ])))
    .with_pre_tokenizer(Some(ByteLevel::default()))
    .with_post_processor(Some(ByteLevel::default()))
    .with_decoder(Some(ByteLevel::default()))
    .build()?;

  let mut trainer = WordLevelTrainerBuilder::default()
    .show_progress(true)
    .special_tokens(special_tokens.to_owned())
    .build()?;

  // train tokenizer on files.
  tokenizer
    .train_from_files(&mut trainer, files.to_owned())?
    .save(&args.save_path, args.pretty)?;
  Ok(())
}
