extern crate lexer;

use lexer::Lexer;

#[test]
fn test_create_lexer() {
    Lexer::new("some string here");
}

#[test]
fn test_empty_line() {
    let mut lexer = Lexer::new("");

    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_one_word() {
    let mut lexer = Lexer::new("word");

    assert_eq!(lexer.next_lexem(), Some("word".to_string()));
}

#[test]
fn test_word_with_space() {
    let mut lexer = Lexer::new("word ");

    assert_eq!(lexer.next_lexem(), Some("word".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
}
