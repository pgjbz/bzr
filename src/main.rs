use bzr::lexer::{Lexer, token::Token};

fn main() {
	let input = String::from("
		let numero int = 5;
		if(numero > 4) {
			var st str = \"manipulado\";
		} else {
			var st str = \"sera?\";
		} 
		let number = 45;
	");
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
