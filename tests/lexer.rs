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
        Token::Lbrace(Location::new(1, 1, FILENAME)),
        Token::Rbrace(Location::new(2, 1, FILENAME)),
        Token::Lparen(Location::new(3, 1, FILENAME)),
        Token::Rparen(Location::new(4, 1, FILENAME)),
        Token::Comma(Location::new(5, 1, FILENAME)),
        Token::Semicolon(Location::new(6, 1, FILENAME)),
        Token::Comma(Location::new(8, 1, FILENAME)),
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
        Token::Ident("abc".to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_identifier_with_number_token() {
    let source = "abc123";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Ident("abc123".to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_invalid_token() {
    let source = "$";

    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal("$".to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_string_token() {
    let source = "\"abc\"";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::String("abc".to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_number_token() {
    let source = "457;\n";
    let mut lexer = Lexer::new(source, FILENAME);
    //TODO: check why is illegal
    assert_eq!(
        Token::Illegal("457".to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_eq_token() {
    let source = "==";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Eq(Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_lte_token() {
    let source = "<=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Lte(Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_token() {
    let source = ">=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Gte(Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_gte_illegal_order() {
    let source = "=>";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal(source.to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_and_operator() {
    let source = "&&";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::And(Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_token() {
    let source = "!=";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Diff(Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}

#[test]
fn test_diff_illegal_order() {
    let source = "=!";
    let mut lexer = Lexer::new(source, FILENAME);
    assert_eq!(
        Token::Illegal(source.to_string(), Location::new(1, 1, FILENAME)),
        *lexer.next_token()
    );
}