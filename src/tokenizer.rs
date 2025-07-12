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
            todo!()
        } else if self.text.starts_with(HIRAGANA_START_CHARS) {
            todo!()
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
