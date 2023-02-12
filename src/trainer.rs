use std::path::Path;
use tokenizers::{
  models::bpe::{BpeTrainerBuilder, BPE},
  normalizers::{strip::Strip, unicode::NFC, utils::Sequence},
  pre_tokenizers::byte_level::ByteLevel,
  AddedToken, Result, TokenizerBuilder,
};

/// Train a Byte-Pair Encoding tokenizer.
pub fn train_bpe(save_path: &Path, files: &[String]) -> Result<()> {
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
    .special_tokens(vec![
      AddedToken::from(String::from("[START]"), true),
      AddedToken::from(String::from("[END]"), true),
      AddedToken::from(String::from("[PAD]"), true),
      AddedToken::from(String::from("[MASK]"), true),
      AddedToken::from(String::from("[UNK]"), true),
    ])
    .build();

  // train tokenizer on files.
  tokenizer
    .train_from_files(&mut trainer, files.to_owned())?
    .save(save_path, true)?;

  Ok(())
}
