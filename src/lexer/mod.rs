use self::token::Token;

pub mod token;

pub struct Lexer<'a> {
	input: &'a str, //Source code
	pub position: usize,
	pub read_position: usize,
	pub ch: Option<char>
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			input,
			position: 0,
			read_position: 0,
			ch: None
		}
	}

	pub fn next_token(&mut self) -> token::Token {
		self.read_char();
		self.skip_whitespace();
		let token = if let Some(ch) = &self.ch {
			match ch {
				'=' => token::Token::Assign(*ch, self.position),
				'+' => token::Token::Plus(*ch, self.position),
				'-' => token::Token::Minus(*ch, self.position),
				'!' => token::Token::Bang(*ch, self.position),
				'/' => token::Token::Slash(*ch, self.position),
				'*' => token::Token::Asterisk(*ch, self.position),
				'<' => token::Token::Lt(*ch, self.position),
				'>' => token::Token::Gt(*ch, self.position),
				';' => token::Token::Semicolon(*ch, self.position),
				'(' => token::Token::Lparen(*ch, self.position),
				')' => token::Token::Rparen(*ch, self.position),
				',' => token::Token::Comma(*ch, self.position),
				'{' => token::Token::Lbrace(*ch, self.position),
				'}' => token::Token::Rbrace(*ch, self.position),
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
				_ => { 
					let read_position = self.read_position;
					let content = self.input;
					if is_letter(Some(*ch)) {
						let ident: &str = Self::read_identifier(self);
						match token::get_keyword_token(&ident,read_position) {
							Ok(keyword_token) => keyword_token,
							Err(_) => token::Token::Ident(String::from(ident), read_position)
						}
					} else if is_number(Some(*ch)) {
						let ident: &str = Self::read_number(self);
						if let Some(ch) = content.chars().nth(read_position + 1) {
							if is_math_simbol(ch) || is_whitespace(Some(ch)) || ch == ';' {
								token::Token::Number(String::from(ident), read_position)
							} else {
								self.read_char();
								token::Token::Illegal(read_position)
							}
						} else {
							token::Token::Illegal(read_position)
						}
					} else {
						token::Token::Illegal(read_position)
					}
				} 
			} 
		} else {
			token::Token::EOF(self.position)
		};
		token
	}

	fn skip_whitespace(&mut self) {
		loop {
			if is_whitespace(self.ch) {
				self.read_char();
			} else {
				break;
			}
		}
	}

	fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = None
		} else {
			// self.ch = self.input[self.read_position];
			self.ch = if let Some(ch) = self.input.chars().nth(self.read_position) {
				Some(ch)
			} else {
				None
			}
		}
		self.position = self.read_position;
		self.read_position = self.read_position + 1;
	}

	fn read_number(input: &mut Self)  -> &str {
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
		while input.position < input.input.len() && input.ch != Some('\"') || input.ch == None {
			input.read_char();
		}
		&input.input[position..input.position+1]
	}

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
		if let  token::Token::EOF(_)  = next {
			None
		} else {
			Some(next)
		}
    }
}

fn is_letter(ch: Option<char>) -> bool {
	if let Some(ch) = ch {
		ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' || ch == '_'
	} else {
		false
	}
}

fn is_number(ch: Option<char>) -> bool {
	if let Some(ch) = ch {
		ch >= '0' && ch <= '9'
	} else {
		false
	}
}

fn is_whitespace(ch: Option<char>) -> bool {
	if let Some(ch) = ch {
		ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
	} else {
		false
	}
}

fn is_math_simbol(ch: char) -> bool {
	ch == '*' || ch == '/' || ch == '+' || ch == '-'
}