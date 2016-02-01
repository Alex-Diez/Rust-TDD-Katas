use std::iter::Peekable;
use std::str::Chars;
use std::string::String;
use std::clone::Clone;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Word(String),

    //characters
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
    Colon
}

impl Clone for Token {

    
}

pub struct Lexer <'a> {
    iter: Peekable<Chars<'a>>,
    previous: Option<Token>
}

impl <'a> Lexer<'a> {
    
    pub fn new(line: &'a str) -> Lexer {
        Lexer {
            iter: line.chars().peekable(),
            previous: None
        }
    }

    fn skip_symbol(&mut self) {
        self.iter.next();
    }

    fn populate_word(&mut self, word: &mut Vec<char>, s: char) {
        self.skip_symbol();
        word.push(s);
    }

    fn evaluate_word(&mut self, word: &mut Vec<char>) -> Option<Token> {
        if word.is_empty() {
            None
        }
        else {
            let string = word.iter().map(|c| *c).collect::<String>().to_lowercase();
            //println!("string {:?}", string);
            self.previous = Some(Token::Word(string.clone()));
            Some(Token::Word(string))
        }
    }

    fn do_token(&mut self, token: Token) -> Option<Token> {
        self.skip_symbol();
        self.previous = Some(token);
        Some(token)
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        let mut value = vec![];
        //println!("previous - {:?}", self.previous);
        loop {
            let symbol = self.iter.peek().map(|c| *c);
            //println!("current symbol - {:?}", symbol);
            match symbol {
                Some(s @ 'a'...'z') | 
                Some(s @ 'A'...'Z') |
                Some(s @ '0'...'9') |
                Some(s @ '_') => self.populate_word(&mut value, s),
                Some('(') => return self.do_token(Token::LeftParenthesis),
                Some(')') => {
                    if !value.is_empty() && self.previous.is_some() {
                        //println!(") break loop");
                        return self.evaluate_word(&mut value)
                    }
                    else {
                        self.iter.next();
                        self.previous = Some(Token::RightParenthesis);
                        return Some(Token::RightParenthesis)
                    }
                },
                Some(';') => {
                    self.iter.next();
                    self.previous = Some(Token::SemiColon);
                    return Some(Token::SemiColon)
                },
                Some('\'') => {
                    if !value.is_empty() && self.previous.is_some() {
                        //println!("' break loop");
                        return self.evaluate_word(&mut value)
                    }
                    else {
                        self.iter.next();
                        self.previous = Some(Token::SingleQuote);
                        return Some(Token::SingleQuote)
                    }
                },
                Some(',') => {
                    if !value.is_empty() && self.previous.is_some() {
                        //println!(", break loop");
                        return self.evaluate_word(&mut value)
                    }
                    else {
                        self.iter.next();
                        self.previous = Some(Token::Colon);
                        return Some(Token::Colon)
                    }
                },
                Some(_) | None => {
                    if !value.is_empty() {
                        self.iter.next();
                        //println!("_ break loop");
                        return self.evaluate_word(&mut value)
                    }
                    else if self.previous.is_some() {
                        self.iter.next();
                        //println!("_ skip");
                    }
                    else {
                        //println!("_ break loop");
                        return self.evaluate_word(&mut value)
                    }
                },
            }
        }
    }
}
