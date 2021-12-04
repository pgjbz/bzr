use std::rc::Rc;

use bzr::{
    ast::expression::Node,
    evaluator,
    lexer::Lexer,
    object::{integer::Integer, Object, boolean::Boolean, string::Str},
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
        assert_eq!(expected, evaluated.val)
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
        assert_eq!(expected, evaluated.val)
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
    evaluator::eval(program.as_ref()).unwrap()
}
