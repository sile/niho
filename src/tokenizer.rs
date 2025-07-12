#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    fn skip_whitespaces(&mut self) {
        self.text = self.text.trim_start();
    }

    fn take_raw_token<F>(&mut self, split: F) -> Token<'a>
    where
        F: FnOnce(&'a str) -> Option<(&'a str, &'a str)>,
    {
        let (text, remaining) = split(self.text).unwrap_or((self.text, ""));
        let token = Token::Raw { text };
        self.text = remaining;
        token
    }

    fn take_hiragana_token(&mut self) -> Token<'a> {
        let mut chars = self.text.char_indices();
        let mut end_pos = self.text.len();

        // Skip the first character (we know it's valid)
        chars.next();

        // Find the end of the hiragana token
        for (pos, ch) in chars {
            if !matches!(
                ch,
                'a'..='z' | ',' | '.' | '?' | '!' | '-' | '[' | ']' | '(' | ')'
            ) {
                end_pos = pos;
                break;
            }
        }

        let text = &self.text[..end_pos];
        self.text = &self.text[end_pos..];
        Token::Hiragana { text }
    }

    fn take_henkan_token(&mut self) -> Token<'a> {
        let mut chars = self.text.char_indices();
        let mut end_pos = self.text.len();
        let mut hash_pos = None;

        // Skip the first character (we know it's uppercase)
        chars.next();

        // Find the end of the henkan token
        for (pos, ch) in chars {
            match ch {
                'a'..='z' | '-' => {
                    // Valid henkan character, continue
                }
                '#' => {
                    // Found hash, now we need to find digits
                    hash_pos = Some(pos);
                    break;
                }
                _ => {
                    // Invalid character, end token here
                    end_pos = pos;
                    break;
                }
            }
        }

        // If we found a hash, we need to parse the number after it
        if let Some(hash_start) = hash_pos {
            let remaining_text = &self.text[hash_start + 1..];
            let mut digit_end = 0;

            for (i, ch) in remaining_text.char_indices() {
                if ch.is_ascii_digit() {
                    digit_end = i + 1;
                } else {
                    break;
                }
            }

            if digit_end > 0 {
                // We have valid digits after the hash
                let text = &self.text[..hash_start];
                let index_str = &remaining_text[..digit_end];
                let index = index_str.parse().unwrap_or(0);

                self.text = &self.text[hash_start + 1 + digit_end..];
                return Token::Henkan { text, index };
            }
        }

        // No valid #number found, treat as raw token up to the end position
        let text = &self.text[..end_pos];
        self.text = &self.text[end_pos..];
        Token::Raw { text }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        if self.text.is_empty() {
            None
        } else if let Some(s) = self.text.strip_prefix("___") {
            self.text = s;
            Some(self.take_raw_token(|s| s.split_once("___")))
        } else if let Some(s) = self.text.strip_prefix("_") {
            self.text = s;
            Some(self.take_raw_token(|s| s.split_once(WHITESPACE_CHARS)))
        } else if self.text.starts_with(HENKAN_START_CHARS) {
            Some(self.take_henkan_token())
        } else if self.text.starts_with(HIRAGANA_START_CHARS) {
            Some(self.take_hiragana_token())
        } else {
            Some(self.take_raw_token(|s| s.split_once(WHITESPACE_CHARS)))
        }
    }
}

const WHITESPACE_CHARS: &[char] = &[' ', '\t', '\n', '\r'];

const HENKAN_START_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const HIRAGANA_START_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '[', ']', '(', ')',
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    // _TEXT(until whitespace char)
    // ___TEXT___
    // non Hiragana or Henkan chars
    Raw { text: &'a str },

    // [a-z\[\]()][a-z,.?!-\[\]()]*
    Hiragana { text: &'a str },

    // [A-Z][a-z-]*(#[0-9]+)
    Henkan { text: &'a str, index: usize },
}
