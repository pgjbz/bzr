use std::{cell::RefCell, env, fs, rc::Rc, process};

use bzr::{evaluator::Evaluator, lexer::Lexer, object::environment::Environment, parser::Parser};

fn main() {
    let filename = if let Some(filename) = env::args().nth(1) {
        filename
    } else {
        eprintln!("please use bzr filename.bzr");
        process::exit(1);
    };
    let input = match fs::read_to_string(&filename) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error on openfile: {:?}", e.kind());
            process::exit(1);
        }
    };

    let lexer = Lexer::new(Rc::new(input), Rc::new(filename));

    let parse = Parser::new(lexer);

    let program = parse.parse_program();
    let eval = Evaluator::default();
    if program.errors.is_empty() {
        eval.eval(
            Some(program.as_ref()),
            Rc::new(RefCell::new(Environment::default())),
        );
    } else {
        for error in program.errors {
            eprintln!("{}", error);
        }
    }
}
