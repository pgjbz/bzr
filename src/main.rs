use std::{env, fs, rc::Rc};

use bzr::{lexer::Lexer, parser::Parser};

fn main() {
    let filename = env::args().nth(1).expect("Expected filename");
    let input = fs::read_to_string(&filename).unwrap();

    let lexer = Lexer::new(Rc::new(input), Rc::new(filename));

    let parse = Parser::new(lexer);

    let program = parse.parse_program();

    if program.errors.is_empty() {
        for sts in program.statements {
            println!("{}", sts.to_string());
        }
    } else {
        for error in program.errors {
            println!("{}", error);
        }
    }
}
