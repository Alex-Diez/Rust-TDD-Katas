use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    query_line_iter: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {

    pub fn new(query_line: &'a str) -> Lexer {
        Lexer { query_line_iter: query_line.chars().peekable() }
    }

    pub fn next_lexem(&mut self) -> Option<String> {
        let mut lexem = vec![];
        loop {
            let symbol = self.query_line_iter.by_ref().peek().cloned();
            match symbol {
                Some(c) => { self.query_line_iter.by_ref().next(); lexem.push(c) },
                None => break,
            }
        }
        if lexem.is_empty() {
            return None;
        }
        Some(lexem.iter().cloned().collect::<String>())
    }
}
