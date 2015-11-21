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
fn test_one_word_line() {
    let mut lexer = Lexer::new("one_word");

    assert_eq!(lexer.next_lexem(), Some("one_word".to_string()));
}

#[test]
fn test_one_word_with_space_line() {
    let mut lexer = Lexer::new("one_word ");

    assert_eq!(lexer.next_lexem(), Some("one_word".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
}

#[test]
fn test_two_words_line() {
    let mut lexer = Lexer::new("one two");

    assert_eq!(lexer.next_lexem(), Some("one".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("two".to_string()));
}

#[test]
fn test_two_words_with_spaces_and_tabulations() {
    let mut lexer = Lexer::new("one \t \t two");

    assert_eq!(lexer.next_lexem(), Some("one".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" \t \t ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("two".to_string()));
}

#[test]
fn test_two_words_with_spaces_and_tabulations_and_new_line() {
    let mut lexer = Lexer::new("one \t \n \t two");

    assert_eq!(lexer.next_lexem(), Some("one".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" \t ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("\n".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" \t ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("two".to_string()));
}
