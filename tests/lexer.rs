use bzr::lexer::{
    token::{Location, Token},
    *,
};

static FILENAME: &'static str = "foo.bzr";

#[test]
fn assert_true() {
    assert!(true);
}

#[test]
fn test_tokens() {
    let source = "{}(),; ,";
    let tokens = vec![
        Token::Lbrace(Some(Location::new(1, 1, FILENAME))),
        Token::Rbrace(Some(Location::new(2, 1, FILENAME))),
        Token::Lparen(Some(Location::new(3, 1, FILENAME))),
        Token::Rparen(Some(Location::new(4, 1, FILENAME))),
        Token::Comma(Some(Location::new(5, 1, FILENAME))),
        Token::Semicolon(Some(Location::new(6, 1, FILENAME))),
        Token::Comma(Some(Location::new(8, 1, FILENAME))),
    ];
    let lexer = Lexer::new(source, FILENAME);

    for (i, token) in lexer.into_iter().enumerate() {
        assert_eq!(*token, tokens[i]);
    }
}

#[test]
fn test_is_whitespace() {
    let source = " \r\t\n";
    let eof = if let Token::EOF(_) = *Lexer::new(source, "foo.bzr").next_token() {
        true
    } else {
        false
    };
    assert!(eof);
}

#[test]
fn test_identifier_token() {
    let source = "abc";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Ident(Some("abc".to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_identifier_with_number_token() {
    let source = "abc123";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Ident(Some("abc123".to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_invalid_token() {
    let source = "$";

    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal(Some("$".to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_string_token() {
    let source = "\"abc\"";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::String(Some("abc".to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_number_token() {
    let source = "457;\0";
    let mut lexer = Lexer::new(source, FILENAME);
    //TODO: check why is illegal
    assert_eq!(
        Token::Number(Some("457".to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_eq_token() {
    let source = "==";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Eq(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_lte_token() {
    let source = "<=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Lte(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_token() {
    let source = ">=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Gte(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_illegal_order() {
    let source = "=>";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal(Some(source.to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_and_operator() {
    let source = "&&";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::And(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_or_operator() {
    let source = "||";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Or(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_token() {
    let source = "!=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Diff(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_illegal_order() {
    let source = "=!";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal(Some(source.to_string()), Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}

#[test]
fn test_while_token() {
    let source = "while";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::While(Some(Location::new(1, 1, FILENAME))),
        *lexer.next_token()
    );
}
