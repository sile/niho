use orfail::OrFail;

use crate::{
    dictionary::{Dictionary, DictionaryEntry, DictionaryString},
    tokenizer::Token,
};
use std::{io::Write, num::NonZeroUsize};

#[derive(Debug, Default)]
pub struct KanaConverter<'a> {
    pub mappings: Vec<KanaMapping<'a>>,
}

impl<'a> KanaConverter<'a> {
    pub fn insert_mapping(
        &mut self,
        from: DictionaryString<'a>,
        to: DictionaryString<'a>,
        consume_chars: Option<NonZeroUsize>,
    ) {
        self.mappings.push(KanaMapping {
            from,
            to,
            consume_chars,
        });
    }

    pub fn finish(&mut self) {
        self.mappings.sort_by(|a, b| a.from.cmp(&b.from));
    }

    pub fn convert<W: Write>(&self, mut writer: W, text: &str) -> orfail::Result<()> {
        let text = text.to_ascii_lowercase();
        let mut s = text.as_str();
        'root: while !s.is_empty() {
            for mapping in self.mappings.iter().rev() {
                if s.starts_with(mapping.from.as_ref()) {
                    write!(writer, "{}", mapping.to).or_fail()?;
                    s = if let Some(n) = mapping.consume_chars {
                        &s[n.get()..]
                    } else {
                        &s[mapping.from.len()..]
                    };
                    continue 'root;
                }
            }

            let (i, c) = s.char_indices().next().expect("infallible");
            write!(writer, "{c}").or_fail()?;
            s = &s[i..];
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct KanaMapping<'a> {
    pub from: DictionaryString<'a>,
    pub to: DictionaryString<'a>,
    pub consume_chars: Option<NonZeroUsize>,
}

#[derive(Debug)]
pub struct Converter<'a> {
    hiragana: KanaConverter<'a>,
    katakana: KanaConverter<'a>,
}

impl<'a> Converter<'a> {
    pub fn new(dic: Dictionary<'a>) -> orfail::Result<Self> {
        let mut hiragana = KanaConverter::default();
        let mut katakana = KanaConverter::default();

        for entry in dic {
            let entry = entry.or_fail()?;
            match entry {
                DictionaryEntry::Hiragana {
                    from,
                    to,
                    consume_chars,
                } => hiragana.insert_mapping(from, to, consume_chars),
                DictionaryEntry::Katakana {
                    from,
                    to,
                    consume_chars,
                } => katakana.insert_mapping(from, to, consume_chars),
                DictionaryEntry::Henkan { from, to } => todo!(),
            }
        }
        hiragana.finish();
        katakana.finish();

        Ok(Self { hiragana, katakana })
    }

    pub fn convert<W: Write>(&self, mut writer: W, token: Token<'_>) -> orfail::Result<()> {
        match token {
            Token::Raw { text } => write!(writer, "{text}").or_fail()?,
            Token::Hiragana { text } => self.hiragana.convert(writer, text).or_fail()?,
            Token::Katakana { text } => self.katakana.convert(writer, text).or_fail()?,
            Token::Henkan { text } => write!(writer, " TODO ").or_fail()?,
        }
        Ok(())
    }
}
