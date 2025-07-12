#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

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
