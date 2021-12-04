use std::rc::Rc;

use bzr::{
    ast::expression::Node,
    evaluator,
    lexer::Lexer,
    object::{integer::Integer, Object},
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

fn test_eval(source: String) -> Box<dyn Object> {
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program: Box<dyn Node> = parser.parse_program();
    evaluator::eval(program.as_ref()).unwrap()
}
