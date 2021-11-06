pub mod token;

pub struct Lexer<'a> {
	input: &'a str, //Source code
	pub position: usize,
	pub read_position: usize,
	pub ch: char
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			input,
			position: 0,
			read_position: 0,
			ch: '0'
		}
	}

	pub fn next_token(&mut self) -> token::Token {
		self.read_char();
		self.skip_whitespace();
		let token = match &self.ch {
			'=' => token::Token::Assign(self.ch),
			'+' => token::Token::Plus(self.ch),
			'-' => token::Token::Minus(self.ch),
			'!' => token::Token::Bang(self.ch),
			'/' => token::Token::Slash(self.ch),
			'*' => token::Token::Asterisk(self.ch),
			'<' => token::Token::Lt(self.ch),
			'>' => token::Token::Gt(self.ch),
			';' => token::Token::Semicolon(self.ch),
			'(' => token::Token::Lparen(self.ch),
			')' => token::Token::Rparen(self.ch),
			',' => token::Token::Comma(self.ch),
			'{' => token::Token::Lbrace(self.ch),
			'}' => token::Token::Rbrace(self.ch),
			'\"' => {
				let string = Self::read_string(self);
				match string.chars().last() {
					Some(ch) => {
						if ch != '\"' {
							token::Token::Illegal
						} else {
							token::Token::String(String::from(&string[0..string.len() - 1]))
						}
					},
					None => token::Token::Illegal
				}
			},
			'\0' => token::Token::EOF,
			_ => 
			if is_letter(self.ch) {
				let ident: &str = Self::read_identifier(self);
				match token::get_keyword_token(&ident) {
					Ok(keyword_token) => keyword_token,
					Err(_) => token::Token::Ident(String::from(ident))
				}
			} else if is_number(self.ch) {
				let ident: &str = Self::read_number(self);
				token::Token::Number(String::from(ident))
			} else {
				token::Token::Illegal
			}
		};

		token
	}

	pub fn skip_whitespace(&mut self) {
		loop {
			if is_whitespace(self.ch) {
				self.read_char();
			} else {
				break;
			}
		}
	}

	pub fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = '\0';
		} else {
			// self.ch = self.input[self.read_position];
			self.ch = if let Some(ch) = self.input.chars().nth(self.read_position) {
				ch
			} else {
				'\0'
			}
		}
		self.position = self.read_position;
		self.read_position = self.read_position + 1;
	}

	fn read_number(input: &mut Self)  -> &str{
		let position = input.position;
		while input.position < input.input.len() && is_number(input.ch) {
			input.read_char();
		}
		let ret = &input.input[position..input.position];
		input.position -= 1;
		input.read_position -= 1;
		ret
	}

	fn read_identifier(input: &mut Self)  -> &str {
		let position = input.position;
		while input.position < input.input.len() && is_letter(input.ch) {
			input.read_char();
		}
		let ret = &input.input[position..input.position];
		input.position -= 1;
		input.read_position -= 1;
		ret
	}

	fn read_string(input: &mut Self) -> &str {
		let position= input.position + 1;
		input.read_char();
		while input.position < input.input.len() && input.ch != '\"' || input.ch == '\0' {
			input.read_char();
		}
		&input.input[position..input.position+1]
	}

}

fn is_letter(ch: char) -> bool {
	ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' || ch == '_'
}

fn is_number(ch: char) -> bool {
	ch >= '0' && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
	ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}