use std::{borrow::Cow, num::NonZeroUsize};

#[derive(Debug)]
pub struct Dictionary<'a> {
    text: &'a str,
    line: usize,
}

impl<'a> Dictionary<'a> {
    pub const DEFAULT: &'static str = include_str!("../default-dic.jsonl");

    pub fn new(text: &'a str) -> Self {
        Self { text, line: 0 }
    }
}

impl<'a> Iterator for Dictionary<'a> {
    type Item = Result<DictionaryEntry<'a>, DictionaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }
        let (current, remaining) = self.text.split_once('\n').unwrap_or((self.text, ""));
        self.text = remaining;

        self.line += 1;
        DictionaryEntry::parse(current)
            .map(Some)
            .map_err(|e| DictionaryError {
                line: self.line,
                reason: e,
            })
            .transpose()
    }
}

pub type DictionaryString<'a> = Cow<'a, str>;

#[derive(Debug)]
pub enum DictionaryEntry<'a> {
    Hiragana {
        from: DictionaryString<'a>,
        to: DictionaryString<'a>,
        consume_chars: Option<NonZeroUsize>,
    },
    Katakana {
        from: DictionaryString<'a>,
        to: DictionaryString<'a>,
        consume_chars: Option<NonZeroUsize>,
    },
    Kanji {
        from: DictionaryString<'a>,
        to: Vec<DictionaryString<'a>>,
    },
}

impl<'a> DictionaryEntry<'a> {
    fn parse(line: &'a str) -> Result<Self, nojson::JsonParseError> {
        let json = nojson::RawJson::parse(line)?;
        let value = json.value();
        match value
            .to_member("type")?
            .required()?
            .to_unquoted_string_str()?
            .as_ref()
        {
            "hiragana" => Ok(Self::Hiragana {
                from: value
                    .to_member("from")?
                    .required()?
                    .to_unquoted_string_str()?,
                to: value
                    .to_member("to")?
                    .required()?
                    .to_unquoted_string_str()?,
                consume_chars: value.to_member("consume_chars")?.try_into()?,
            }),
            "katakana" => Ok(Self::Katakana {
                from: value
                    .to_member("from")?
                    .required()?
                    .to_unquoted_string_str()?,
                to: value
                    .to_member("to")?
                    .required()?
                    .to_unquoted_string_str()?,
                consume_chars: value.to_member("consume_chars")?.try_into()?,
            }),
            "kanji" => Ok(Self::Kanji {
                from: value
                    .to_member("from")?
                    .required()?
                    .to_unquoted_string_str()?,
                to: value.to_member("to")?.required()?.try_into()?,
            }),
            ty => Err(value.invalid(format!("unknown type: {ty:?}"))),
        }
    }
}

#[derive(Debug)]
pub struct DictionaryError {
    pub line: usize,
    pub reason: nojson::JsonParseError,
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dictionary parse error at line {}: {}",
            self.line, self.reason
        )
    }
}

impl std::error::Error for DictionaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}
