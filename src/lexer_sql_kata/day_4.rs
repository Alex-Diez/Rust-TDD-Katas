use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
enum SymbolGroup {
    AlphaNumeric,
    WhiteSpace,
    Else
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {
    
    pub fn new(line: &'a str) -> Lexer {
        Lexer { iter: line.chars().peekable() }
    }

    pub fn next_lexem(&mut self) -> Option<String> {
        let mut value = vec![];
        let expected = self.define_symbol_group();
        if expected == SymbolGroup::Else {
            return None;
        }
        loop {
            let actual = self.define_symbol_group();
            let symbol = self.peek_next_symbol();
            if expected == actual {
                self.iter.next();
                value.push(symbol.unwrap());
            }
            else {
                break;
            }
        }
        Some(value.iter().cloned().collect::<String>())
    }

    fn define_symbol_group(&mut self) -> SymbolGroup {
        match self.peek_next_symbol() {
            Some('a' ...'z') | Some('A'...'Z') |
            Some('_') | Some('0'...'9') => SymbolGroup::AlphaNumeric,
            Some(' ') => SymbolGroup::WhiteSpace,
            Some(_) | None => SymbolGroup::Else,
        }
    }

    fn peek_next_symbol(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }
}
