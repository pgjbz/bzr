use std::{env, fs, rc::Rc};

use bzr::{lexer::Lexer, parser::Parser};

fn main() {
    let filename = env::args().nth(1).expect("Expected filename");
    let input = fs::read_to_string(&filename).unwrap();
    
    let mut lexer = Lexer::new(Rc::new(input), Rc::new(filename));
    // for tok in &mut lexer {
    //     println!("{:?}", tok);
    // }
    let mut parse = Parser::new(&mut lexer);

    let val = parse.parse_let_sts();
    if let Some(val) = val {
        println!("{}", val)
    } else {
        println!("Fake")
    }
}
