use std::{borrow::Cow, num::NonZeroUsize};

#[derive(Debug)]
pub struct Dictionary<'a> {
    text: &'a str,
    line: usize,
}

impl<'a> Dictionary<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, line: 0 }
    }
}

impl<'a> Iterator for Dictionary<'a> {
    type Item = Result<DictionaryEntry<'a>, DictionaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Default for Dictionary<'static> {
    fn default() -> Self {
        Self::new(include_str!("../default-dic.jsonl"))
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
    Henkan {
        from: DictionaryString<'a>,
        to: DictionaryString<'a>,
    },
}

#[derive(Debug)]
pub enum DictionaryError {}
