pub use tdd_kata::lexer_sql_kata::day_4::Lexer;

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
fn test_one_word_line_differ_case_and_underscore_plus_numbers() {
    let mut lexer = Lexer::new("1_Word_Some_2_Where_3_Here");

    assert_eq!(lexer.next_lexem(), Some("1_Word_Some_2_Where_3_Here".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_words_devided_by_white_spaces() {
    let mut lexer = Lexer::new("Word Some Where Here");

    assert_eq!(lexer.next_lexem(), Some("Word".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("Some".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("Where".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("Here".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}

#[test]
fn test_sql_with_comment_line() {
    let mut lexer = Lexer::new("Word //Some Comment Here\nAnother Word");

    assert_eq!(lexer.next_lexem(), Some("Word".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("Another".to_string()));
    assert_eq!(lexer.next_lexem(), Some(" ".to_string()));
    assert_eq!(lexer.next_lexem(), Some("Word".to_string()));
    assert_eq!(lexer.next_lexem(), None);
}
