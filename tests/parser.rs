use std::rc::Rc;

use bzr::{lexer::Lexer, parser::Parser};

#[test]
fn test_parse_let_int_type() {
    let source = "let a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_int_type() {
    let source = "var a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_let_negative_int_type() {
    let source = "let a int = -10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_negative_int_type() {
    let source = "var a int = -10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_int_type_erros() {
    let source = "var a int = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_let_int_type_erros() {
    let source = "var a int = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_var_int_letter_type_erros() {
    let source = "var a int = 10a".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_let_int_letter_type_erros() {
    let source = "var a int = 10a".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_str_int_type() {
    let source = "let a str = \"10\";".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_str_type() {
    let source = "var a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_str_type_erros() {
    let source = "var a str = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_let_str_type_erros() {
    let source = "var a int = \"10\"".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

//

#[test]
fn test_parse_bool_int_type() {
    let source = "let a str = \"10\";".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_bool_type() {
    let source = "var a bool = false;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() <= 0);
    assert!(progrma.statements.len() > 0);
}

#[test]
fn test_parse_var_bool_type_erros() {
    let source = "var a bool = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}

#[test]
fn test_parse_let_bool_type_erros() {
    let source = "var a bool = false".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let progrma = parser.parse_program();
    assert!(progrma.errors.len() > 0);
    assert!(progrma.statements.len() <= 0);
}
