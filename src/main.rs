use std::fs;

use bzr::lexer::{Lexer};

fn main() {
	let input = fs::read_to_string("foo.bzr").unwrap();
	let mut lexer = Lexer::new(&input);
	// lexer.read_char();
	for token in &mut lexer {
		println!("{:?}", token);
	}
}
