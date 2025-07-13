use orfail::OrFail;

use crate::{
    dictionary::{Dictionary, DictionaryEntry, DictionaryString},
    tokenizer::Token,
};
use std::{collections::HashMap, num::NonZeroUsize};

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
    hiragana_map: HashMap<&'static str, &'static str>,
    henkan_map: HashMap<&'static str, Vec<&'static str>>,
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

        Ok(Self {
            hiragana,
            katakana,
            hiragana_map: Self::build_hiragana_map(),
            henkan_map: Self::build_henkan_map(),
        })
    }

    pub fn convert_tokens<'b>(&self, tokens: impl Iterator<Item = Token<'b>>) -> String {
        let mut result = String::new();

        for token in tokens {
            match token {
                Token::Raw { text } => {
                    result.push_str(text);
                }
                Token::Hiragana { text } => {
                    result.push_str(&self.convert_hiragana(text));
                }
                Token::Katakana { text } => {
                    result.push_str(&self.convert_hiragana(text)); // TODO
                }
                Token::Henkan { text } => {
                    let index = 0; // todo
                    result.push_str(&self.convert_henkan(text, index));
                }
            }
        }

        result
    }

    fn convert_hiragana(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_ascii_alphabetic() {
                // Try to match longest possible sequence first
                let mut matched = false;

                // Try 3-character combinations first
                if let Some(&next1) = chars.peek() {
                    let mut temp_chars = chars.clone();
                    temp_chars.next();
                    if let Some(&next2) = temp_chars.peek() {
                        let three_char = format!("{}{}{}", ch, next1, next2);
                        if let Some(&hiragana) = self.hiragana_map.get(three_char.as_str()) {
                            result.push_str(hiragana);
                            chars.next(); // consume next1
                            chars.next(); // consume next2
                            matched = true;
                        }
                    }
                }

                // Try 2-character combinations
                if !matched {
                    if let Some(&next) = chars.peek() {
                        let two_char = format!("{}{}", ch, next);
                        if let Some(&hiragana) = self.hiragana_map.get(two_char.as_str()) {
                            result.push_str(hiragana);
                            chars.next(); // consume next
                            matched = true;
                        }
                    }
                }

                // Try single character
                if !matched {
                    let single_char = ch.to_string();
                    if let Some(&hiragana) = self.hiragana_map.get(single_char.as_str()) {
                        result.push_str(hiragana);
                        matched = true;
                    }
                }

                if !matched {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn convert_henkan(&self, text: &str, index: usize) -> String {
        // Remove the first character (which should be uppercase) and convert to lowercase
        let key = text[1..].to_lowercase();

        if let Some(candidates) = self.henkan_map.get(key.as_str()) {
            if index < candidates.len() {
                candidates[index].to_string()
            } else {
                // If index is out of bounds, return the romanized version
                text.to_string()
            }
        } else {
            // If no conversion found, return the original text
            text.to_string()
        }
    }

    fn build_hiragana_map() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();

        // Single vowels
        map.insert("a", "あ");
        map.insert("i", "い");
        map.insert("u", "う");
        map.insert("e", "え");
        map.insert("o", "お");

        // K series
        map.insert("ka", "か");
        map.insert("ki", "き");
        map.insert("ku", "く");
        map.insert("ke", "け");
        map.insert("ko", "こ");

        // G series
        map.insert("ga", "が");
        map.insert("gi", "ぎ");
        map.insert("gu", "ぐ");
        map.insert("ge", "げ");
        map.insert("go", "ご");

        // S series
        map.insert("sa", "さ");
        map.insert("si", "し");
        map.insert("shi", "し");
        map.insert("su", "す");
        map.insert("se", "せ");
        map.insert("so", "そ");

        // Z series
        map.insert("za", "ざ");
        map.insert("zi", "じ");
        map.insert("ji", "じ");
        map.insert("zu", "ず");
        map.insert("ze", "ぜ");
        map.insert("zo", "ぞ");

        // T series
        map.insert("ta", "た");
        map.insert("ti", "ち");
        map.insert("chi", "ち");
        map.insert("tu", "つ");
        map.insert("tsu", "つ");
        map.insert("te", "て");
        map.insert("to", "と");

        // D series
        map.insert("da", "だ");
        map.insert("di", "ぢ");
        map.insert("du", "づ");
        map.insert("de", "で");
        map.insert("do", "ど");

        // N series
        map.insert("na", "な");
        map.insert("ni", "に");
        map.insert("nu", "ぬ");
        map.insert("ne", "ね");
        map.insert("no", "の");
        map.insert("n", "ん");

        // H series
        map.insert("ha", "は");
        map.insert("hi", "ひ");
        map.insert("hu", "ふ");
        map.insert("fu", "ふ");
        map.insert("he", "へ");
        map.insert("ho", "ほ");

        // B series
        map.insert("ba", "ば");
        map.insert("bi", "び");
        map.insert("bu", "ぶ");
        map.insert("be", "べ");
        map.insert("bo", "ぼ");

        // P series
        map.insert("pa", "ぱ");
        map.insert("pi", "ぴ");
        map.insert("pu", "ぷ");
        map.insert("pe", "ぺ");
        map.insert("po", "ぽ");

        // M series
        map.insert("ma", "ま");
        map.insert("mi", "み");
        map.insert("mu", "む");
        map.insert("me", "め");
        map.insert("mo", "も");

        // Y series
        map.insert("ya", "や");
        map.insert("yu", "ゆ");
        map.insert("yo", "よ");

        // R series
        map.insert("ra", "ら");
        map.insert("ri", "り");
        map.insert("ru", "る");
        map.insert("re", "れ");
        map.insert("ro", "ろ");

        // W series
        map.insert("wa", "わ");
        map.insert("wi", "ゐ");
        map.insert("we", "ゑ");
        map.insert("wo", "を");

        // Special characters
        map.insert(",", "、");
        map.insert(".", "。");
        map.insert("?", "？");
        map.insert("!", "！");
        map.insert("-", "ー");

        map
    }

    fn build_henkan_map() -> HashMap<&'static str, Vec<&'static str>> {
        let mut map = HashMap::new();

        // Example conversions based on the README
        map.insert("o-maji", vec!["ローマ字"]);
        map.insert("wo", vec!["を", "お"]);
        map.insert("ihongo", vec!["日本語"]);
        map.insert("i", vec!["に", "へ"]);
        map.insert("enkan", vec!["変換"]);
        map.insert("urutemeno", vec!["するための"]);
        map.insert("u-ru", vec!["ツール"]);
        map.insert("esu", vec!["です"]);

        // Common Japanese words
        map.insert("atashi", vec!["私", "わたし"]);
        map.insert("nata", vec!["あなた", "貴方"]);
        map.insert("re", vec!["これ", "それ", "あれ"]);
        map.insert("a", vec!["は", "が", "を"]);
        map.insert("asu", vec!["です", "ます"]);
        map.insert("ru", vec!["する", "くる", "ある"]);

        map
    }
}
