pub use tdd_kata::lexer_sql_kata::day_6::{Lexer, Token};

#[test]
fn test_create_lexer() {
    Lexer::new("some line here");
}

#[test]
fn test_empty_line() {
    let mut lexer = Lexer::new("");

    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_insert_keyword() {
    let mut lexer = Lexer::new("iNseRt");

    assert_eq!(lexer.next_lexem(), Some(Token::Insert));
}

#[test]
fn test_into_keyword() {
    let mut lexer = Lexer::new("into");

    assert_eq!(lexer.next_lexem(), Some(Token::Into));
}

#[test]
fn test_identifier() {
    let mut lexer = Lexer::new("ident");

    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("ident".to_string())));
}

#[test]
fn test_values_keyword() {
    let mut lexer = Lexer::new("values");

    assert_eq!(lexer.next_lexem(), Some(Token::Values));
}

#[test]
fn test_left_parenthesis() {
    let mut lexer = Lexer::new("(");

    assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
}

#[test]
fn test_right_parenthesis() {
    let mut lexer = Lexer::new(")");

    assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
}

#[test]
fn test_semicolon() {
    let mut lexer = Lexer::new(";");

    assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
}

#[test]
fn test_single_quote() {
    let mut lexer = Lexer::new("'");

    assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
}

#[test]
fn test_insert_query() {
    let mut lexer = Lexer::new("insert into tab1 values (1, '1');");

    assert_eq!(lexer.next_lexem(), Some(Token::Insert));
    assert_eq!(lexer.next_lexem(), Some(Token::Into));
    assert_eq!(lexer.next_lexem(), Some(Token::Values));
    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("tab1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::Values));
    assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::Constant("1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::Colon));
    assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
    assert_eq!(lexer.next_lexem(), Some(Token::Constant("1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
    assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::SemiColon));
}
