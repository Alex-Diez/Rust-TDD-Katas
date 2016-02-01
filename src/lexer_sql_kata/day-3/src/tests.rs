extern crate lexer;

use lexer::Lexer;

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
fn test_one_word_line() {
    let mut lexer = Lexer::new("word");

    assert_eq!(lexer.next_lexem(), Some("word".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_one_word_line_with_upper_case() {
    let mut lexer = Lexer::new("wOrd");

    assert_eq!(lexer.next_lexem(), Some("wOrd".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_one_word_line_with_numbers() {
    let mut lexer = Lexer::new("w0rd123456789");

    assert_eq!(lexer.next_lexem(), Some("w0rd123456789".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_one_word_line_with_underscore() {
    let mut lexer = Lexer::new("w0rd_");

    assert_eq!(lexer.next_lexem(), Some("w0rd_".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_one_word_with_space() {
    let mut lexer = Lexer::new("word ");

    assert_eq!(lexer.next_lexem(), Some("word".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_two_word_line_with_banch_of_white_spaces() {
    let mut lexer = Lexer::new("one \t\t   \t \t two");

    assert_eq!(lexer.next_lexem(), Some("one".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" \t\t   \t \t ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("two".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_two_words_devided_by_dot() {
    let mut lexer = Lexer::new("one.two");

    assert_eq!(lexer.next_lexem(), Some("one".to_string()));
    assert_eq!(lexer.next_lexem(), Some(".".to_string()));
    assert_eq!(lexer.next_lexem(), Some("two".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_simple_inser_query() {
    let mut lexer = Lexer::new("insert into tab1 values('1');");

    assert_eq!(lexer.next_lexem(), Some("insert".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("into".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("tab1".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("values".to_string()));
    assert_eq!(lexer.next_lexem(), Some("(".to_string()));
    assert_eq!(lexer.next_lexem(), Some("'".to_string()));
    assert_eq!(lexer.next_lexem(), Some("1".to_string()));
    assert_eq!(lexer.next_lexem(), Some("'".to_string()));
    assert_eq!(lexer.next_lexem(), Some(")".to_string()));
    assert_eq!(lexer.next_lexem(), Some(";".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}
