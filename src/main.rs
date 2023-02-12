use clap::Parser;
use tokenizers::{Result, Tokenizer};
use wikitext::Cli;

fn main() -> Result<()> {
  let args = Cli::parse();
  println!("{args:?}");

  if args.train {
    let files: Vec<String> = std::fs::read_dir(&args.data_dir)
      .unwrap()
      .map(|f| f.unwrap().path().to_str().unwrap().to_string())
      .collect();

    println!("Training {} tokenizer on {files:?}", &args.tokenizer);

    wikitext::train(&args)?;
  } else if args.save_path.is_file() {
    let tokenizer = Tokenizer::from_file(&args.save_path)?;
    // TODO: Add as a cli argument.
    let sentence = "Testing out my new tokenizer.";
    let output = if let Some(sentence) = args.sentence.as_deref() {
      tokenizer.encode(sentence, true)
    } else {
      tokenizer.encode("The quick brown fox jumps over the lazy dog.", true)
    }?;

    println!("Sentence: {sentence}");
    println!("Tokens: {:?}", output.get_tokens());
    println!("IDS: {:?}", output.get_ids());
  } else {
    eprintln!("{} not found.", &args.save_path.display());
  }

  Ok(())
}
