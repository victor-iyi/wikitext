<!--
 Copyright (c) 2023 Victor I. Afolabi

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# wikitext

[![CI](https://github.com/victor-iyi/wikitext/actions/workflows/ci.yml/badge.svg)](https://github.com/victor-iyi/wikitext/actions/workflows/ci.yml)

## Donwload the data

Download the [wikitext-103] (516M of text) dataset.

[wikitext-103]: https://blog.einstein.ai/the-wikitext-long-term-dependency-language-modeling-dataset/

```sh
# Download data.
wget -P data/ https://s3.amazonaws.com/research.metamind.io/wikitext/wikitext-103-raw-v1.zip

# Unzip data.
unzip data/wikitext-103-raw-v1.zip -d data/
```

## Usage

<!-- markdownlint-disable MD013 -->

```sh
$ cargo r -- --help
Train and perform NLP tasks on the wikitext-103 dataset

Usage: wikitext [OPTIONS]

Options:
  -d, --data-dir <DATA_DIR>
          Path to the train, test & valid data
          
          [default: data/wikitext-103-raw]

  -s, --save-path <SAVE_PATH>
          Path to save trained tokenizer
          
          [default: data/wikitext-tokenizer.json]

      --sentence <SENTENCE>
          Sentence to encode with trained tokenizer

      --tokenizer <TOKENIZER>
          List of possible tokenizer algorithms to use
          
          [default: bpe]

          Possible values:
          - bpe:
            One of the most popular subword tokenization algorithm. The Byte-Pair-Encoding works by statring with characters, while merging those that are the most frequently seen to
gether, thus creating new tokens
          - unigram:
            Unigram is also a subword tokenization algorithm, and works by trying to identify the best set of subword tokens to maximize the probability for a given sentence
          - word-level:
            This is the "classic" tokenization algorithm. It simply map words to IDs without anything fancy
          - word-piece:
            This is a subword tokenization algorithm quite similar to BPE, used mainly by Google in models like BERT. It uses a greedy algorithm, that tries to build long words first
, splitting in mujltiple tokens when entire words don't exit in the vocabulary

      --pre-tokenizer <PRE_TOKENIZER>
          List of possible pre-tokenizer rules to use
          
          [default: byte-level]

          Possible values:
          - byte-level:
            Splits on whitespace while remapping all the bytes to a set of visible characters
          - whitespace:
            Splits on word boundaries (using regular expression: \w+|[^\w\s]+

      --train
          Train the tokenizer

      --pretty
          Prettify trainer output

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Train the tokenizer

To train a BPE tokenizer on the [wikitext-103] dataset, run:

```sh
cargo r -- \
  --train \
  --tokenizer bpe \
  --data-dir data/wikitext-103-raw/ \
  --save-path data/wikitext-tokenizer.json
```

You can train other kinds of tokenizer.

## Use the tokenizer

To use the trained tokenizer to encode new sentence, run:

```sh
cargo r -- \
  --tokenizer bpe \
  --save-path data/wikitext-tokenizer.json \
  --sentence "The quick brown fox jumps over the lazy dog."
```

## Contribution

You are very welcome to modify and use them in your own projects.

Please keep a link to the [original repository]. If you have made a fork with
substantial modifications that you feel may be useful, then please [open a new
issue on GitHub][issues] with a link and short description.

## License (MIT)

This project is opened under the [MIT][license] which allows very
broad use for both private and commercial purposes.

A few of the images used for demonstration purposes may be under copyright.
These images are included under the "fair usage" laws.

[original repository]: https://github.com/victor-iyi/wikitext
[issues]: https://github.com/victor-iyi/wikitext/issues
[license]: ./LICENSE
