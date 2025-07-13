use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Dictionary<R> {
    reader: R,
    line_buf: String,
    line_number: usize,
}

impl<R: BufRead> Iterator for Dictionary<R> {
    type Item = Result<DictionaryEntry, DictionaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_buf.clear();
        match self.reader.read_line(&mut self.line_buf) {
            Err(e) => todo!(),
            Ok(0) => None,
            Ok(_) => {
                self.line_number += 1;
                todo!()
            }
        }
    }
}

impl Default for Dictionary<BufReader<&'static [u8]>> {
    fn default() -> Self {
        Self {
            line_buf: String::new(),
            line_number: 0,
            reader: BufReader::new(include_str!("../default-dic.jsonl").as_bytes()),
        }
    }
}

#[derive(Debug)]
pub enum DictionaryEntry {
    Hiragana,
    Katakana,
    Henkan,
}

#[derive(Debug)]
pub enum DictionaryError {}
