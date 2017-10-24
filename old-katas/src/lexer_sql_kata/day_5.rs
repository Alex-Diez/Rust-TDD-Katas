use std::iter::Peekable;
use std::str::Chars;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    Insert,
    Into,
    Values,
    LeftParenthesis,
    RightParenthesis,
    SingleQuote,
    Colon,
    Constant(String),
    Semicolon
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    insert: Regex,
    into: Regex,
    values: Regex
}

impl <'a> Lexer<'a> {

    pub fn new(line: &'a str) -> Lexer {
        Lexer {
            iter: line.chars().peekable(),
            insert: regex!("(?i)insert"),
            into: regex!("(?i)into"),
            values: regex!("(?i)values")
        }
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        let mut value = vec![];
        loop {
            let symbol = self.iter.peek().cloned();
            match symbol {
                Some(s @ 'a'...'z') | Some(s @ 'A'...'Z') |
                Some(s @ '_') | Some(s @ '0'...'9') => { self.iter.next(); value.push(s) },
                Some(_) | None => break,
            }
        }
        println!("value size - {:?}", value.len());
        if value.is_empty() {
            None
        }
        else {
            let string = value.iter().cloned().collect::<String>();
            if self.insert.is_match(&string) {
                Some(Token::Insert)
            }
            else if self.into.is_match(&string) {
                Some(Token::Into)
            }
            else if self.values.is_match(&string) {
                Some(Token::Values)
            }
            else {
                Some(Token::Identifier(string))
            }
        }
    }
}
