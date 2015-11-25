#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::iter::Peekable;
use std::str::Chars;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    Constant(String),

    //keywords
    Insert,
    Into,
    Values,

    //characters
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
    Colon
}

pub struct Lexer <'a> {
    iter: Peekable<Chars<'a>>,
    insert: Regex,
    into: Regex,
    values: Regex,
    previous: Option<Token>
}

impl <'a> Lexer<'a> {
    
    pub fn new(line: &'a str) -> Lexer {
        Lexer {
            iter: line.chars().peekable(),
            insert: regex!("(?i)insert"),
            into: regex!("(?i)into"),
            values: regex!("(?i)values"),
            previous: None
        }
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        let mut value = vec![];
        println!("previous - {:?}", self.previous);
        loop {
            let symbol = self.iter.peek().map(|c| *c);
            match symbol {
                Some(s @ 'a'...'z') | Some(s @ 'A'...'Z') |
                Some(s @ '0'...'1') => { self.iter.next(); value.push(s) },
                Some('(') => { self.iter.next(); return Some(Token::LeftParenthesis) },
                Some(')') => { self.iter.next(); return Some(Token::RightParenthesis) },
                Some(';') => { self.iter.next(); return Some(Token::SemiColon) },
                Some('\'') => { self.iter.next(); return Some(Token::SingleQuote) },
                Some(',') => { self.iter.next(); return Some(Token::Colon) },
                Some(_) | None => { if self.previous.is_some() { println!("skip symbol {:?}", symbol); self.iter.next(); } else { println!("break loop"); break }  },
            }
        }
        if value.is_empty() {
            None
        }
        else {
            let string = value.iter().map(|c| *c).collect::<String>();
            if self.insert.is_match(&string) {
                self.previous = Some(Token::Insert);
                Some(Token::Insert)
            }
            else if self.into.is_match(&string) {
                self.previous = Some(Token::Into);
                Some(Token::Into)
            }
            else if self.values.is_match(&string) {
                self.previous = Some(Token::Values);
                Some(Token::Values)
            }
            else {
                self.previous = Some(Token::Identifier(string.clone()));
                Some(Token::Identifier(string))
            }
        }
    }
}
