use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Word(String),

    //characters
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    SingleQuote,
    Colon
}

pub struct Lexer <'a> {
    iter: Peekable<Chars<'a>>,
    previous: Option<Token>,
    current_symbol: Option<char>,
    previous_symbol: Option<char>
}

impl <'a> Lexer<'a> {
    
    pub fn new(line: &'a str) -> Lexer {
        Lexer {
            iter: line.chars().peekable(),
            previous: None,
            current_symbol: None,
            previous_symbol: None,
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
            let string = word.iter().cloned().collect::<String>().to_lowercase();
            self.previous = Some(Token::Word(string.clone()));
            Some(Token::Word(string))
        }
    }

    fn do_token(&mut self, token: Token) -> Option<Token> {
        self.skip_symbol();
        self.previous = Some(token.clone());
        Some(token)
    }

    pub fn next_lexem(&mut self) -> Option<Token> {
        let mut value = vec![];
        loop {
            if self.previous_symbol != Some('\'') && self.current_symbol != Some('\'') {
                self.previous_symbol = self.current_symbol;
                self.current_symbol = self.iter.peek().cloned();
            }
            println!("68 previous - {:?} current - {:?}", self.previous_symbol, self.current_symbol);
            match self.current_symbol {
                Some(s @ 'a'...'z') | 
                Some(s @ 'A'...'Z') |
                Some(s @ '0'...'9') |
                Some(s @ '_') => self.populate_word(&mut value, s),
                Some('(') => return self.do_token(Token::LeftParenthesis),
                Some(')') => {
                    if !value.is_empty() && self.previous.is_some() {
                        return self.evaluate_word(&mut value)
                    }
                    else {
                        return self.do_token(Token::RightParenthesis)
                    }
                },
                Some(';') => return self.do_token(Token::SemiColon),
                Some('\'') => {
                    
                    if !value.is_empty() && self.previous.is_some() {
                        self.previous_symbol = self.current_symbol;
                        self.skip_symbol();
                        self.current_symbol = self.iter.peek().cloned();
                        match self.previous_symbol {
                            Some('A'...'Z') |
                            Some('a'...'z') |
                            Some('0'...'9') |
                            Some('_') => {
                                println!("94 previous - {:?} current - {:?}", self.previous_symbol, self.current_symbol);
                                if self.current_symbol != Some('\'') && self.previous_symbol == Some('\'') {
                                    let c = self.previous_symbol.unwrap();
                                    self.populate_word(&mut value, c)
                                }
                            },
                            Some(' ') => return self.do_token(Token::SingleQuote),
                            Some(_) | None => return self.evaluate_word(&mut value),
                        }
                    }
                    else {
                        return self.do_token(Token::SingleQuote)
                    }
                },
                Some(',') => {
                    if !value.is_empty() && self.previous.is_some() {
                        return self.evaluate_word(&mut value)
                    }
                    else {
                        return self.do_token(Token::Colon)
                    }
                },
                Some(_) | None => {
                    if !value.is_empty() {
                        self.skip_symbol();
                        return self.evaluate_word(&mut value)
                    }
                    else if self.previous.is_some() {
                        self.skip_symbol();
                    }
                    else {
                        return self.evaluate_word(&mut value)
                    }
                },
            }
        }
    }
}
