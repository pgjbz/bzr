use self::token::{Location, Token};

pub mod token;

pub struct Lexer<'a> {
	input: &'a str, //Source code
	position: usize,
	read_position: usize,
	ch: Option<char>,
	line: usize,
	line_position: usize,
	filename: &'static str
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str, filename: &'static str) -> Self {
		Self {
			input,
			position: 0,
			read_position: 0,
			ch: None,
			line: 1,
			line_position: 0,
			filename
		}
	}

	pub fn next_token(&mut self) -> Token {
		self.read_char();
		self.skip_whitespace();
		let location = Location::new(self.line_position, self.line, self.filename);
		let token = if let Some(ch) = &self.ch {
			match ch {
				'=' => Token::Assign(location),
				'+' => Token::Plus(location),
				'-' => Token::Minus(location),
				'!' => Token::Bang(location),
				'/' => Token::Slash(location),
				'*' => Token::Asterisk(location),
				'<' => Token::Lt(location),
				'>' => Token::Gt(location),
				';' => Token::Semicolon(location),
				'(' => Token::Lparen(location),
				')' => Token::Rparen(location),
				',' => Token::Comma(location),
				'{' => Token::Lbrace(location),
				'}' => Token::Rbrace(location),
				'\"' => {
					let string = Self::read_string(self);
					match string.chars().last() {
						Some(ch) => {
							if ch != '\"' {
								Token::Illegal(location)
							} else {
								Token::String(String::from(&string[0..string.len() - 1]), location)
							}
						},
						None => Token::Illegal(location)
					}
				},
				_ => { 
					let read_position = self.read_position;
					let content = self.input;
					if is_letter(Some(*ch)) {
						let ident: &str = Self::read_identifier(self);
						match Token::get_keyword_token(&ident, location.clone()) {
							Ok(keyword_token) => keyword_token,
							Err(_) => Token::Ident(String::from(ident), location)
						}
					} else if is_number(Some(*ch)) {
						let ident: &str = Self::read_number(self);
						if let Some(ch) = content.chars().nth(read_position + 1) {
							if is_math_simbol(ch)
							|| is_whitespace(Some(ch)) 
							|| ch == ';'
							|| ch == '{' {
								Token::Number(String::from(ident), location)
							} else {
								self.read_char();
								Token::Illegal(location)
							}
						} else {
							Token::Illegal(location)
						}
					} else {
						Token::Illegal(location)
					}
				} 
			} 
		} else {
			Token::EOF(location)
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
		self.ch =self.input.chars().nth(self.read_position);
		
		if let Some(ch) = self.ch {
			if ch == '\n' {
				self.line += 1;
				self.line_position = 0;
			} else {
				self.line_position += 1;
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
		Self::back_position(input);
		ret
	}

	fn read_identifier(input: &mut Self)  -> &str {
		let position = input.position;
		while input.position < input.input.len() && is_letter(input.ch) {
			input.read_char();
		}
		let ret = &input.input[position..input.position];
		Self::back_position(input);
		ret
	}

	fn back_position(input: &mut Self) {
		input.line_position -= 1;
		input.position -= 1;
		input.read_position -= 1;
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
	ch == '*' 
	|| ch == '/' 
	|| ch == '+' 
	|| ch == '-' 
	|| ch == ')' 
	|| ch == '(' 
}