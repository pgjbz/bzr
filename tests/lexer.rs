use std::rc::Rc;

use bzr::lexer::{
    token::{Location, Token},
    *,
};

const FILENAME: &'static str = "foo.bzr";

#[test]
fn assert_true() {
    assert!(true);
}

#[test]
fn test_tokens() {
    let source = Rc::new("{}(),; ,".to_string());
    let tokens = vec![
        Token::Lbrace(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        Token::Rbrace(Some(Location::new(1, 1, Rc::new(FILENAME.to_string())))),
        Token::LParen(Some(Location::new(2, 1, Rc::new(FILENAME.to_string())))),
        Token::RParen(Some(Location::new(3, 1, Rc::new(FILENAME.to_string())))),
        Token::Comma(Some(Location::new(4, 1, Rc::new(FILENAME.to_string())))),
        Token::Semicolon(Some(Location::new(5, 1, Rc::new(FILENAME.to_string())))),
        Token::Comma(Some(Location::new(7, 1, Rc::new(FILENAME.to_string())))),
    ];
    let lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));

    for (i, token) in lexer.into_iter().enumerate() {
        assert_eq!(*token, tokens[i]);
    }
}

#[test]
fn test_is_whitespace() {
    let source = Rc::new(" \r\t\n".to_string());
    let eof = if let Token::EOF(_) = *Lexer::new(source, Rc::new(FILENAME.to_string())).next_token()
    {
        true
    } else {
        false
    };
    assert!(eof);
}

#[test]
fn test_identifier_token() {
    let source = Rc::new("abc".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Ident(
            Some(source),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_identifier_with_number_token() {
    let source = Rc::new("abc123".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Ident(
            Some(source),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_invalid_token() {
    let source = Rc::new("$".to_string());

    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Illegal(
            Some(source),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_string_token() {
    let source = Rc::new("\"abc\"".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::String(
            Some(Rc::new("abc".to_string())),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_number_token() {
    let source = Rc::new("457;\0".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Number(
            Some(Rc::new("457".to_string())),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_eq_token() {
    let source = Rc::new("==".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Eq(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_lte_token() {
    let source = Rc::new("<=".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Lte(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_token() {
    let source = Rc::new(">=".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Gte(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_illegal_order() {
    let source = Rc::new("=>".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Illegal(
            Some(source),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_and_operator() {
    let source = Rc::new("&&".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::And(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_or_operator() {
    let source = Rc::new("||".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Or(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_token() {
    let source = Rc::new("!=".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Diff(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_illegal_order() {
    let source = Rc::new("=!".to_string());
    let mut lexer = Lexer::new(Rc::clone(&source), Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::Illegal(
            Some(source),
            Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))
        ),
        *lexer.next_token()
    );
}

#[test]
fn test_while_token() {
    let source = Rc::new("while".to_string());
    let mut lexer = Lexer::new(source, Rc::new(FILENAME.to_string()));
    assert_eq!(
        Token::While(Some(Location::new(0, 1, Rc::new(FILENAME.to_string())))),
        *lexer.next_token()
    );
}
