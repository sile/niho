#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    fn take_sonomama_token<F>(&mut self, split: F) -> Token<'a>
    where
        F: FnOnce(&'a str) -> Option<(&'a str, &'a str)>,
    {
        let (text, remaining) = split(self.text).unwrap_or((self.text, ""));
        self.text = remaining;
        Token::Sonomama { text }
    }

    fn take_hiragana_or_katakana_token(&mut self) -> Token<'a> {
        let pattern = |c: char| c == '_' || c.is_ascii_whitespace();
        let pos = self.text.find(pattern).unwrap_or(self.text.len());
        let text = &self.text[..pos];
        let underscore_count = self.text[pos..].chars().take_while(|&c| c == '_').count();
        self.text = &self.text[pos + underscore_count..];
        self.text = self.text.strip_prefix(' ').unwrap_or(self.text);
        if let Some(index) = underscore_count.checked_sub(1) {
            Token::Kanji { text, index }
        } else if text
            .trim_start_matches(|c: char| !c.is_ascii_alphabetic())
            .starts_with(|c: char| c.is_ascii_uppercase())
        {
            Token::Katakana { text }
        } else {
            Token::Hiragana { text }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            None
        } else if self.text.starts_with(|c: char| c.is_ascii_whitespace()) {
            Some(self.take_sonomama_token(|s| {
                s.find(|c: char| !c.is_ascii_whitespace())
                    .map(|pos| s.split_at(pos))
            }))
        } else if let Some(s) = self.text.strip_prefix('_') {
            self.text = s;
            Some(self.take_sonomama_token(|s| s.split_once(|c: char| c.is_ascii_whitespace())))
        } else {
            Some(self.take_hiragana_or_katakana_token())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Sonomama { text: &'a str },
    // Dictionary JSON: {"type": "hiragana", "from": "ka", "to": "か"}
    Hiragana { text: &'a str },
    // Dictionary JSON: {"type": "katakana", "from": "ka", "to": "カ"}
    Katakana { text: &'a str },
    // Dictionary JSON: {"type": "kanji", "from": "にほんご", "to": ["日本語"]}
    Kanji { text: &'a str, index: usize },
}

impl<'a> nojson::DisplayJson for Token<'a> {
    fn fmt(&self, f: &mut nojson::JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.object(|f| match self {
            Token::Sonomama { text } => {
                f.member("type", "sonomama")?;
                f.member("text", text)
            }
            Token::Hiragana { text } => {
                f.member("type", "hiragana")?;
                f.member("text", text)
            }
            Token::Katakana { text } => {
                f.member("type", "katakana")?;
                f.member("text", text)
            }
            Token::Kanji { text, index } => {
                f.member("type", "kanji")?;
                f.member("text", text)?;
                f.member("index", index)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_handling() {
        let tokens = Tokenizer::new("  foo bar  baz\n")
            .map(text)
            .collect::<Vec<_>>();
        assert_eq!(tokens, ["  ", "foo", "bar", " ", "baz", "\n"]);
    }

    fn text<'a>(token: Token<'a>) -> &'a str {
        match token {
            Token::Sonomama { text }
            | Token::Hiragana { text }
            | Token::Katakana { text }
            | Token::Kanji { text, .. } => text,
        }
    }
}
