use std::{env, fs};

use bzr::lexer::Lexer;

fn main() {
    let filename = env::args().nth(1).expect("Expected filename");
    let input = fs::read_to_string(&filename).unwrap();
    let mut lexer = Lexer::new(&input, &filename);
    for token in &mut lexer {
        println!("{:?}", token)
    }
}
