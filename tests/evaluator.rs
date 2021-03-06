use std::{cell::RefCell, rc::Rc};

use bzr::{
    ast::expression::Node,
    evaluator::Evaluator,
    lexer::Lexer,
    object::{
        array::Array, boolean::Boolean, environment::Environment, function::Function,
        integer::Integer, string::Str, Object,
    },
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
        let value = evaluated.val;
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
        let value = evaluated.val;
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
        let value = evaluated.val;
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

fn test_eval(source: String) -> Rc<dyn Object> {
    let lexer = Lexer::new(Rc::new(source), Rc::new("foo.bzr".to_string()));
    let parser = Parser::new(lexer);
    let program: Box<dyn Node> = parser.parse_program();
    let eval = Evaluator::default();
    let env = RefCell::new(Environment::default());
    eval.eval(Some(program.as_ref()), Rc::new(env)).unwrap()
}

#[test]
fn test_bang_operator_boolean() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("!false".to_string(), true));
    tests.push(("!true".to_string(), false));
    tests.push(("!!false".to_string(), false));
    tests.push(("!!true".to_string(), true));

    for (source, expected) in tests {
        println!("{}", source);
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.val;
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
        let value = evaluated.val;
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
    tests.push(("true || false;".to_string(), true));
    tests.push(("true && true;".to_string(), true));
    tests.push(("true && false;".to_string(), false));
    tests.push(("10 == 10;".to_string(), true));
    tests.push(("10 != 10;".to_string(), false));
    tests.push(("10 > 10;".to_string(), false));
    tests.push(("10 < 10;".to_string(), false));
    tests.push(("10 >= 5;".to_string(), true));
    tests.push(("5 <= 10;".to_string(), true));
    tests.push(("1 != 2;".to_string(), true));
    tests.push(("(2 > 1) == true;".to_string(), true));
    tests.push(("(2 < 1) == false;".to_string(), true));
    tests.push(("(1 < 2) == true;".to_string(), true));
    tests.push(("(1 > 2) == false;".to_string(), true));
    tests.push(("(1 > 2) || false;".to_string(), false));
    tests.push(("(1 > 2) || true;".to_string(), true));
    tests.push(("(1 > 2) || (2 < 1);".to_string(), false));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_if_expr() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("if false { 1 } else { 10 } ".to_string(), 10));
    tests.push(("if 1 < 2 { 10 } else { 1 }".to_string(), 10));
    tests.push(("if true { 10 }".to_string(), 10));
    tests.push(("if 1 > 2 { 10 } else { 100 }".to_string(), 100));
    tests.push(("let x = 10; if 1 > x { 10 } else { 100 }".to_string(), 100));
    tests.push((
        "if 1 > 2 { 10; } else if 10 == 10 { 100; } else { 45; }".to_string(),
        100,
    ));
    tests.push((
        "if 1 > 2 { 10; } else if 10 != 10 { 100; } else { 45; }".to_string(),
        45,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_return_expr() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("ret 10;".to_string(), 10));
    tests.push(("ret 10; 9;".to_string(), 10));
    tests.push(("ret 2 * 5; 9;".to_string(), 10));
    tests.push(("5 * 5 * 5; ret 10; 10 * 10;".to_string(), 10));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_errors() {
    let mut tests: Vec<(String, &str)> = Vec::new();
    tests.push(("5 + true;".to_string(), "incompatible types bool and int"));
    tests.push(("-true;".to_string(), "invalid expression '-true'"));
    tests.push((
        "true + false;".to_string(),
        "unsupported operation true + false",
    ));
    tests.push((
        "5; true + false;".to_string(),
        "unsupported operation true + false",
    ));
    tests.push((
        "\"Hello\" - \" World\";".to_string(),
        "unsupported operation Hello -  World",
    ));
    tests.push(("foobar".to_string(), "unknown word 'foobar'"));
    tests.push((
        "if 1 == 1 { let a = 10; } a;".to_string(),
        "unknown word 'a'",
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated
            .as_any()
            .downcast_ref::<bzr::object::error::Error>()
            .unwrap();
        let value = evaluated.val.clone();
        assert_eq!(expected, value)
    }
}

#[test]
fn test_let_statement() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("let a int = 5; a;".to_string(), 5));
    tests.push(("let a = 5; a;".to_string(), 5));
    tests.push(("let a = 5; let b = 10; a;".to_string(), 5));
    tests.push(("let a int = 5; let b = 20; a;".to_string(), 5));
    tests.push((
        "let a int = 5; let b = 20;
    if a > b { 10; } else { 5; }"
            .to_string(),
        5,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_var_statement() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("var a int = 5; a;".to_string(), 5));
    tests.push(("var a = 5; a;".to_string(), 5));
    tests.push(("var a = 5; var b = 10; a;".to_string(), 5));
    tests.push(("var a int = 5; var b = 20; a;".to_string(), 5));
    tests.push((
        "var a int = 5; var b = 20;
    if a > b { 10; } else { 5; }"
            .to_string(),
        5,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_function_expr() {
    let source = "fn x_plus_two(x int) { x + 2; }".to_string();

    let evaluated = test_eval(source);
    let evaluated = evaluated.as_any().downcast_ref::<Function>();
    assert!(evaluated.is_some(), "Not a function");
}

#[test]
fn test_function_apply() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push((
        "fn x_plus_two(x int) {  x + 2; } x_plus_two(10);".to_string(),
        12,
    ));
    tests.push(("fn identity(x int) { x; } identity(10);".to_string(), 10));
    tests.push(("fn add(a int, b int) { a + b; } add(5, 5);".to_string(), 10));
    tests.push((
        "fn add(a int, b int) { ret a + b; } add(5, 5);".to_string(),
        10,
    ));
    tests.push((
        "fn identity(x int) { ret x; } identity(10);".to_string(),
        10,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_factorial() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push((
        "fn factorial(x int) int {
            if x <= 1 {
                ret 1;
            } else {
                ret x * factorial(x - 1);
            }
        }
        
        factorial(4);"
            .to_string(),
        24,
    ));
    tests.push((
        "fn factorial(x int) int {
        if x <= 1 {
            ret 1;
        } else {
            ret x * factorial(x - 1);
        }
    }
    
    factorial(0);"
            .to_string(),
        1,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_closure() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push((
        "fn calculate(a int, b int, p int) int {
            ret p(a, b);
        }
        
        fn add(a int, b int) int {
            ret a + b;
        }
        
        calculate(5, 5, add);"
            .to_string(),
        10,
    ));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_string_concatenation() {
    let source = "\"Paulo\" + \" \" + \"Gabriel\"".to_string();
    let evaluated = test_eval(source);
    let evaluated = evaluated.as_any().downcast_ref::<Str>();
    assert!(evaluated.is_some(), "Not a string");
    assert_eq!("Paulo Gabriel".to_string(), evaluated.unwrap().val)
}

#[test]
fn test_built_in() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("len(\"abc\")".to_string(), 3));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_set_value() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("let a int = 10; a = 0; a;".to_string(), 0));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}

#[test]
fn test_eval_array() {
    let source = "[1, 2 * 2, 3 + 3]".to_string();
    let evaluated = test_eval(source);
    let evaluated = evaluated.as_any().downcast_ref::<Array>();
    assert!(evaluated.is_some(), "Not a array");
    assert_eq!(
        evaluated.unwrap().elements.borrow_mut().len(),
        3,
        "Wrong size"
    );
}

#[test]
fn test_array_index() {
    let mut tests: Vec<(String, i64)> = Vec::new();
    tests.push(("let a = [1, 2, 3]; a[0]".to_string(), 1));
    tests.push(("let a = [1, 2 * 2, 3 + 3]; a[1]".to_string(), 4));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.val;
        assert_eq!(expected, value)
    }
}
