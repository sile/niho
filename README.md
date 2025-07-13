niho
====

[![niho](https://img.shields.io/crates/v/niho.svg)](https://crates.io/crates/niho)
[![Actions Status](https://github.com/sile/niho/workflows/CI/badge.svg)](https://github.com/sile/niho/actions)
![License](https://img.shields.io/crates/l/niho)

`niho` is a command-line tool for converting romanized Japanese text to Japanese characters.

```console
$ echo _niho ha, Ro-ma ji_ wo nihongo_ni henkan_surutameno Tu-ru desu. | niho
nihoは、ローマ字を日本語に変換するためのツールです。
```

## Installation

```console
$ cargo install niho
A command-line tool for converting romanized Japanese text to Japanese characters

Usage: niho [OPTIONS]

Options:
      --version                Print version
  -h, --help                   Print help ('--help' for full help, '-h' for summary)
  -t, --tokenize               Output tokenized input as JSON instead of converting to Japanese
  -d, --dictionary-file <PATH> Path to dictionary file [env: NIHO_DICTIONARY_FILE]
```

## Basic Syntax

`niho` converts romanized Japanese text to Japanese characters using the following syntax:

### Input Text Types

- **Regular text**: Converted to hiragana (e.g., `konnnichiwa` → `こんにちは`)
- **Capitalized text**: Converted to katakana (e.g., `Ko-hi-` → `コーヒー`)
- **Text ending with `_`**: Converted to kanji using dictionary lookup (e.g., `nihongo_` → `日本語`)
- **Text wrapped in `___`**: Kept as raw text without conversion (e.g., `___Hello___` → `Hello`)
- **Text prefixed with `_`**: Kept as raw text until whitespace (e.g., `_Hello desu` → `Hello です`)

### Examples

```console
# Convert hiragana
$ echo konnnichiwa | niho
こんにちは

# Convert katakana (use uppercase)
$ echo Ko-hi- | niho
コーヒー

# Convert kanji (use underscore suffix)
$ echo nihongo_ | niho
日本語

# Mix different types
$ echo watashi ha Ko-hi- wo nomimasu | niho
わたしはコーヒーをのみます

# Keep raw text
$ echo ___English to ___ nihongo_ | niho
English to 日本語
```

## Dictionary Format

The dictionary is stored in a JSONL (JSON Lines) format, where each line contains a JSON object representing a character or word mapping. The dictionary contains three types of entries:

- **`hiragana`**: Maps romanized text to hiragana characters
- **`katakana`**: Maps romanized text to katakana characters
- **`kanji`**: Maps romanized words to kanji characters

Example entries:
```json
{"type": "hiragana", "from": "ka", "to": "か"}
{"type": "katakana", "from": "ka", "to": "カ"}
{"type": "kanji", "from": "nihongo", "to": "日本語"}
```

The default dictionary can be found at [default-dic.jsonl](default-dic.jsonl).

### Kanji and Unknown Word Handling

For kanji conversion (text ending with `_`), the tool performs a direct dictionary lookup:

- **Found words**: If the romanized text exists in the kanji dictionary, it's converted to the corresponding kanji (e.g., `nihongo_` → `日本語`)
- **Unknown words**: If no mapping is found, the text is wrapped in angle brackets to indicate it's unrecognized (e.g., `unknown_` → `<unknown>`)
