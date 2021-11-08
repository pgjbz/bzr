use std::fs;

use bzr::lexer::{Lexer};

fn main() {
	let filename = "foo.bzr";
	let input = fs::read_to_string(filename).unwrap();
	let mut lexer = Lexer::new(&input, filename);
	for token in &mut lexer {
		println!("{:?}", token);
	}

}
