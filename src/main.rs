use bzr::lexer::{Lexer, token::Token};

fn main() {
	let input = String::from("let abestado = 5;");
	let mut lexer = Lexer::new(input.chars().collect());
	// lexer.read_char();
	loop {
		let token = lexer.next_token();
		if token == Token::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
	}
}
