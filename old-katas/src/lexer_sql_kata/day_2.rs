use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
enum SymbolGroup {
    AlphaNumeric,
    WhiteSpace,
    NewLine,
    Else
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {
    
    pub fn new(line: &'a str) -> Lexer {
        Lexer {
            iter: line.chars().peekable()
        }
    }

    pub fn next_lexem(&mut self) -> Option<String> {
        let mut value = vec![];
        let expected = self.define_symbol_group();
        if expected == SymbolGroup::Else {
            return None;
        }
        loop {
            let actual = self.define_symbol_group();
            let symbol = self.next_symbol();
            if !self.do_on_expected_symbol(&expected, &actual, &mut value, symbol) {
                break;
            }
        }
        Some(value.iter().cloned().collect::<String>())
    }

    fn next_symbol(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    fn define_symbol_group(&mut self) -> SymbolGroup {
        let symbol = self.next_symbol();
        match symbol {
            Some('a'...'z') | Some('A'...'Z') |
            Some('0'...'9') | Some('_') => SymbolGroup::AlphaNumeric,
            Some(' ') | Some('\t') => SymbolGroup::WhiteSpace,
            Some('\n') | Some('\r') => SymbolGroup::NewLine,
            Some(_) | None => SymbolGroup::Else,
        }
    }

    fn add_symbol(&mut self, value: &mut Vec<char>, s: char) {
        self.iter.next();
        value.push(s);
    }

    fn do_on_expected_symbol(&mut self, expected: &SymbolGroup, actual: &SymbolGroup, value: &mut Vec<char>, s: Option<char>) -> bool {
        if *expected == *actual {
            self.add_symbol(value, s.unwrap());
            true
        }
        else {
            false
        }
    }
}