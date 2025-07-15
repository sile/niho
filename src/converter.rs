use orfail::OrFail;

use crate::{
    dictionary::{Dictionary, DictionaryEntry, DictionaryString},
    tokenizer::Token,
};
use std::{collections::HashMap, io::Write, num::NonZeroUsize};

#[derive(Debug)]
pub struct Converter<'a> {
    hiragana: KanaConverter<'a>,
    katakana: KanaConverter<'a>,
    kanji: KanjiConverter<'a>,
}

impl<'a> Converter<'a> {
    pub fn new(dic: Dictionary<'a>) -> orfail::Result<Self> {
        let mut hiragana = KanaConverter::default();
        let mut katakana = KanaConverter::default();
        let mut kanji = KanjiConverter::default();
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
                DictionaryEntry::Kanji { from, to } => kanji.insert_mapping(from, to),
            }
        }
        hiragana.finish();
        katakana.finish();

        Ok(Self {
            hiragana,
            katakana,
            kanji,
        })
    }

    pub fn convert<W: Write>(&self, mut writer: W, token: Token<'_>) -> orfail::Result<()> {
        match token {
            Token::Raw { text } => write!(writer, "{text}").or_fail()?,
            Token::Hiragana { text } => self.hiragana.convert(writer, text).or_fail()?,
            Token::Katakana { text } => self.katakana.convert(writer, text).or_fail()?,
            Token::Kanji { text, count } => self.kanji.convert(writer, text, count).or_fail()?,
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct KanaConverter<'a> {
    mappings: Vec<KanaMapping<'a>>,
}

impl<'a> KanaConverter<'a> {
    fn insert_mapping(
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

    fn finish(&mut self) {
        self.mappings.sort_by(|a, b| a.from.cmp(&b.from));
    }

    fn convert<W: Write>(&self, mut writer: W, text: &str) -> orfail::Result<()> {
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

            let c = s.chars().next().expect("infallible");
            write!(writer, "{c}").or_fail()?;
            s = &s[c.len_utf8()..];
        }
        Ok(())
    }
}

#[derive(Debug)]
struct KanaMapping<'a> {
    from: DictionaryString<'a>,
    to: DictionaryString<'a>,
    consume_chars: Option<NonZeroUsize>,
}

#[derive(Debug, Default)]
struct KanjiConverter<'a> {
    mappings: HashMap<DictionaryString<'a>, DictionaryString<'a>>,
}

impl<'a> KanjiConverter<'a> {
    fn insert_mapping(&mut self, from: DictionaryString<'a>, to: DictionaryString<'a>) {
        self.mappings.insert(from, to);
    }

    fn convert<W: Write>(
        &self,
        mut writer: W,
        text: &str,
        count: Option<isize>,
    ) -> orfail::Result<()> {
        if let Some(s) = self.mappings.get(text) {
            match count {
                None => write!(writer, "{s}").or_fail()?,
                Some(count) if count >= 0 => {
                    for c in s.chars().take(count as usize) {
                        write!(writer, "{c}").or_fail()?;
                    }
                }
                Some(count) => {
                    for c in s
                        .chars()
                        .skip(s.chars().count().saturating_sub(count.unsigned_abs()))
                    {
                        write!(writer, "{c}").or_fail()?;
                    }
                }
            }
        } else {
            write!(writer, "<{text}>").or_fail()?;
        }
        Ok(())
    }
}
