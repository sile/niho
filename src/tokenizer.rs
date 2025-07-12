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
        let pattern = |c| matches!(c, 'A'..='Z' | ' ');
        let pos = self.text.find(pattern).unwrap_or(self.text.len());
        let text = &self.text[..pos];
        self.text = &self.text[pos..];
        Token::Hiragana { text }
    }

    fn take_henkan_token(&mut self) -> Token<'a> {
        let pattern = |c| !matches!(c, 'a'..='z' | '-');
        let pos = self.text[1..].find(pattern).unwrap_or(self.text.len() - 1) + 1;
        let text = &self.text[..pos];
        self.text = &self.text[pos..];

        let mut index = 0;

        if let Some(remaining) = self.text.strip_prefix('#') {
            let pos = remaining
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(remaining.len());
            index = remaining[..pos].parse().unwrap_or(0);
            self.text = &remaining[pos..];
        }

        Token::Henkan { text, index }
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

const WHITESPACE_CHARS: &[char] = &[' '];

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

impl<'a> nojson::DisplayJson for Token<'a> {
    fn fmt(&self, f: &mut nojson::JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.object(|f| match self {
            Token::Raw { text } => {
                f.member("type", "Raw")?;
                f.member("text", text)
            }
            Token::Hiragana { text } => {
                f.member("type", "Hiragana")?;
                f.member("text", text)
            }
            Token::Henkan { text, index } => {
                f.member("type", "Henkan")?;
                f.member("text", text)?;
                f.member("index", index)
            }
        })
    }
}
