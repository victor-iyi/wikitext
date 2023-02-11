<!--
 Copyright (c) 2023 Victor I. Afolabi

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# wikitext

[![CI](https://github.com/victor-iyi/wikitext/actions/workflows/ci.yml/badge.svg)](https://github.com/victor-iyi/wikitext/actions/workflows/ci.yml)

Download the [wikitext-103] (516M of text) dataset.

[wikitext-103]: https://blog.einstein.ai/the-wikitext-long-term-dependency-language-modeling-dataset/

```sh
# Download data.
wget -P data/ https://s3.amazonaws.com/research.metamind.io/wikitext/wikitext-103-raw-v1.zip

# Unzip data.
cd data/
unzip wikitext-103-raw-v1.zip
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
