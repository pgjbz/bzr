use std::rc::Rc;

use bzr::{
    ast::expression::Node,
    evaluator,
    lexer::Lexer,
    object::{boolean::Boolean, integer::Integer, string::Str, Object},
    parser::Parser,
};

#[test]
fn test_eval_integer() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("5".to_string(), 5));
    tests.push(("10".to_string(), 10));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_integer_minus_operator() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("5".to_string(), 5));
    tests.push(("-5".to_string(), -5));
    tests.push(("10".to_string(), 10));
    tests.push(("-10".to_string(), -10));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_boolean() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("false".to_string(), false));
    tests.push(("true".to_string(), true));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_str() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push(("\"false\"".to_string(), "false".to_string()));
    tests.push(("\"true\"".to_string(), "true".to_string()));
    tests.push(("\"5\"".to_string(), "5".to_string()));
    tests.push(("\"10\"".to_string(), "10".to_string()));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Str>().unwrap();
        assert_eq!(expected, evaluated.val)
    }
}

fn test_eval(source: String) -> Box<dyn Object> {
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program: Box<dyn Node> = parser.parse_program();
    evaluator::eval(Some(program.as_ref())).unwrap()
}

#[test]
fn test_bang_operator_boolean() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("!false".to_string(), true));
    tests.push(("!true".to_string(), false));
    tests.push(("!!false".to_string(), false));
    tests.push(("!!true".to_string(), true));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_infix_number_expr() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("10 + 10".to_string(), 20));
    tests.push(("10 - 10".to_string(), 0));
    tests.push(("10 + 2 * 5".to_string(), 20));
    tests.push(("5 + 5 + 5 + 5 - 10".to_string(), 10));
    tests.push(("2 * (5 + 10)".to_string(), 30));
    tests.push(("(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(), 50));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_infix_bool_expr() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("false != false;".to_string(), false));
    tests.push(("false == false;".to_string(), true));
    tests.push(("true != false;".to_string(), true));
    tests.push(("true != true;".to_string(), false));
    tests.push(("true == true;".to_string(), true));
    tests.push(("10 == 10;".to_string(), true));
    tests.push(("10 != 10;".to_string(), false));
    tests.push(("10 > 10;".to_string(), false));
    tests.push(("10 < 10;".to_string(), false));
    tests.push(("10 >= 5;".to_string(), true));
    tests.push(("5 <= 10;".to_string(), true));
    tests.push(("1 != 2;".to_string(), true));

    for (source, expected) in tests {
        println!("{}", source);
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = *evaluated.val.borrow_mut();
        assert_eq!(expected, value)
    }
}
