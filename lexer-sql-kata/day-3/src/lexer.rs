use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
enum SymbolGroup {
    AlphaNumeric,
    WhiteSpace,
    Dot,
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
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
            if expected == actual {
                value.push(self.iter.next().unwrap());
            }
            else {
                break;
            }
        }
        Some(value.iter().map(|c| *c).collect::<String>())
    }

    fn define_symbol_group(&mut self) -> SymbolGroup {
        let symbol = self.peek_next_symbol();
        match symbol {
            Some('a'...'z') | Some('A'...'Z') |
            Some('0'...'9') | Some('_') => SymbolGroup::AlphaNumeric,
            Some('.') => SymbolGroup::Dot,
            Some('(') => SymbolGroup::LeftParenthesis,
            Some(')') => SymbolGroup::RightParenthesis,
            Some(';') => SymbolGroup::SemiColon,
            Some('\'') => SymbolGroup::SingleQuote,
            Some(' ') | Some('\t') => SymbolGroup::WhiteSpace,
            Some(_) | None => SymbolGroup::Else,
        }
    }

    fn peek_next_symbol(&mut self) -> Option<char> {
        self.iter.peek().map(|c| *c)
    }
}