pub use tdd_kata::lexer_sql_kata::day_5::{Lexer, Token};

#[test]
fn test_create_lexer() {
    Lexer::new("some line here");
}

#[test]
fn test_empty_lexem() {
    let mut lexer = Lexer::new("");

    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_identifier_lexem() {
    let mut lexer = Lexer::new("OnE_123");

    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("OnE_123".to_string())));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
#[ignore]
fn test_insert_query() {
    let mut lexer = Lexer::new("insert into tab1(col1, col2) values ('1', 2);");

    assert_eq!(lexer.next_lexem(), Some(Token::Insert));
    assert_eq!(lexer.next_lexem(), Some(Token::Into));
    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("tab1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("col1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::Colon));
    assert_eq!(lexer.next_lexem(), Some(Token::Identifier("col2".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::Values));
    assert_eq!(lexer.next_lexem(), Some(Token::LeftParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
    assert_eq!(lexer.next_lexem(), Some(Token::Constant("1".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::SingleQuote));
    assert_eq!(lexer.next_lexem(), Some(Token::Colon));
    assert_eq!(lexer.next_lexem(), Some(Token::Constant("2".to_string())));
    assert_eq!(lexer.next_lexem(), Some(Token::RightParenthesis));
    assert_eq!(lexer.next_lexem(), Some(Token::Semicolon));
}
