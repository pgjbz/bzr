use std::fs;

use bzr::lexer::{Lexer, token::Token};

fn main() {
	let input = fs::read_to_string("foo.bzr").unwrap();
	let mut lexer = Lexer::new(&input);
	// lexer.read_char();
	loop {
		let token = lexer.next_token();
		if let Token::EOF(_) = token {
            break;
        } else {
            println!("{:?}", token);
        }
	}
}
