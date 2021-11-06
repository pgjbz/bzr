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
			'=' => token::Token::Assign(self.ch, self.position),
			'+' => token::Token::Plus(self.ch, self.position),
			'-' => token::Token::Minus(self.ch, self.position),
			'!' => token::Token::Bang(self.ch, self.position),
			'/' => token::Token::Slash(self.ch, self.position),
			'*' => token::Token::Asterisk(self.ch, self.position),
			'<' => token::Token::Lt(self.ch, self.position),
			'>' => token::Token::Gt(self.ch, self.position),
			';' => token::Token::Semicolon(self.ch, self.position),
			'(' => token::Token::Lparen(self.ch, self.position),
			')' => token::Token::Rparen(self.ch, self.position),
			',' => token::Token::Comma(self.ch, self.position),
			'{' => token::Token::Lbrace(self.ch, self.position),
			'}' => token::Token::Rbrace(self.ch, self.position),
			'\"' => {
				let position = self.position;
				let string = Self::read_string(self);
				match string.chars().last() {
					Some(ch) => {
						if ch != '\"' {
							token::Token::Illegal(position)
						} else {
							token::Token::String(String::from(&string[0..string.len() - 1]), position)
						}
					},
					None => token::Token::Illegal(position)
				}
			},
			'\0' => token::Token::EOF(self.position),
			_ => { 
				let position = self.position;
				if is_letter(self.ch) {
					let ident: &str = Self::read_identifier(self);
					match token::get_keyword_token(&ident,position) {
						Ok(keyword_token) => keyword_token,
						Err(_) => token::Token::Ident(String::from(ident), position)
					}
				} else if is_number(self.ch) {
					let ident: &str = Self::read_number(self);
					token::Token::Number(String::from(ident), position)
				} else {
					token::Token::Illegal(position)
				}
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