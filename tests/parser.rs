use bzr::ast::{
    expr::if_expr::IfExpr, expression::Expression, stmt::expression_stmt::ExpressionStatement,
};
use std::rc::Rc;

use bzr::{
    ast::{
        expr::int_expr::IntExpr, identifier::Identifier, program::Program, statement::Statement,
        stmt::let_stmt::Let, types::Type,
    },
    lexer::{token::Token, Lexer},
    parser::Parser,
};

#[test]
fn test_parse_let_int_type() {
    let source = "let a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_int_type() {
    let source = "var a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_let_negative_int_type() {
    let source = "let a int = -10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_negative_int_type() {
    let source = "var a int = -10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_int_type_erros() {
    let source = "var a int = \"texto\"".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_let_int_type_erros() {
    let source = "var a int = \"10\"".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_var_int_letter_type_erros() {
    let source = "var a int = 10a".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_let_int_letter_type_erros() {
    let source = "var a int = 10a".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_str_int_type() {
    let source = "let a str = \"10\";".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_str_type() {
    let source = "var a int = 10;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_str_type_erros() {
    let source = "var a str = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_let_str_type_erros() {
    let source = "var a int = \"10\"".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

//

#[test]
fn test_parse_bool_int_type() {
    let source = "let a str = \"10\";".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_bool_type() {
    let source = "var a bool = false;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_parse_var_bool_type_erros() {
    let source = "var a bool = 10".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_let_bool_type_erros() {
    let source = "var a bool = \"false\"".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() > 0);
    assert!(program.statements.len() <= 0);
}

#[test]
fn test_parse_return_type() {
    let source = "ret false;".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.errors.len() <= 0);
    assert!(program.statements.len() > 0);
}

#[test]
fn test_program_to_string() {
    let token = Rc::new(Token::Let(None));
    let typ = Type::Int;
    let identifier = Box::new(Identifier::new(
        Rc::new("my_var".to_string()),
        Rc::new(Token::Ident(None, None)),
    ));
    let expr: Box<dyn Expression> = Box::new(IntExpr::new(10, Rc::new(Token::Number(None, None))));
    let statements: Vec<Box<dyn Statement>> = vec![Let::new(token, typ, identifier, expr)];
    let program = Program::new(statements, vec![]);
    let program_str = program.to_string();
    assert_eq!("let my_var int = 10;", program_str);
}

#[test]
fn test_parse_precedence() {
    let source = "-a * b;
    !-a;
    a + b + c;
    a + b - c;
    a * b * c;
    a * b / c;
    a + b / c;
    a + b * c + d / e - f;
    3 + 4; -5 * 5;
    5 > 4 == 3 < 4;
    5 < 4 != 3 > 4;
    3 + 4 * 5 == 3 * 1 + 4 * 5;
    3 + 4 * 5 == 3 * 1 + 4 * 5;
    10 >= 10;
    20 <= 50;
    10 + 2 * 3
    3 > 5 == false
    3 < 5 == true
    true == true
    false == false
    false != true
    false == true
    !(true == true)"
        .to_string();
    let expected = "((-a) * b)
(!(-a))
((a + b) + c)
((a + b) - c)
((a * b) * c)
((a * b) / c)
(a + (b / c))
(((a + (b * c)) + (d / e)) - f)
(3 + 4)
(-5 * 5)
((5 > 4) == (3 < 4))
((5 < 4) != (3 > 4))
((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))
((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))
(10 >= 10)
(20 <= 50)
(10 + (2 * 3))
((3 > 5) == false)
((3 < 5) == true)
(true == true)
(false == false)
(false != true)
(false == true)
(!(true == true))
"
    .to_string();

    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let mut output = String::new();
    let program = parser.parse_program();
    for stmt in program.statements {
        output.push_str(&stmt.to_string());
        output.push_str("\n");
    }
    assert_eq!(&expected, &output);
}

#[test]
fn test_if_expression() {
    let source = "if x < y { x }".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
    let stmt = program.statements.first().unwrap();
    let expr_stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();
    let exists = expr_stmt
        .expression
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IfExpr>()
        .unwrap()
        .consequence
        .is_some();
    assert!(exists);
}

#[test]
fn test_if_else_expression() {
    let source = "if x < y { x } else { y }".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
    let stmt = program.statements.first().unwrap();
    let expr_stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();
    let exists = expr_stmt
        .expression
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IfExpr>()
        .unwrap()
        .alternative
        .is_some();
    assert!(exists);
}

#[test]
fn test_if_else_if_expression() {
    let source = "if x < y { x } else if a > b { a } else { y }".to_string();
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
}
