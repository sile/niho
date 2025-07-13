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

    fn take_hiragana_or_henkan_token(&mut self) -> Token<'a> {
        let pattern = |c: char| c == '_' || c.is_ascii_whitespace();
        let pos = self.text.find(pattern).unwrap_or(self.text.len());
        let text = &self.text[..pos];
        let is_henkan = self.text[pos..].starts_with('_');
        self.text = &self.text[pos + 1..];
        if is_henkan {
            Token::Henkan { text }
        } else {
            Token::Hiragana { text }
        }
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
            Some(self.take_raw_token(|s| s.split_once(|c: char| c.is_ascii_whitespace())))
        } else {
            Some(self.take_hiragana_or_henkan_token())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Raw { text: &'a str },
    Hiragana { text: &'a str },
    Henkan { text: &'a str },
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
            Token::Henkan { text } => {
                f.member("type", "Henkan")?;
                f.member("text", text)
            }
        })
    }
}
