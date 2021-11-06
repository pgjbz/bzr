#[derive(PartialEq, Debug)]
pub enum Token {
	Illegal,
    EOF,
    Ident(Vec<char>),
    Int(Vec<char>),
    Assign(char),
    Plus(char),
    Comma(char),
    Semicolon(char),
    Lparen(char),
    Rparen(char),
    Lbrace(char),
    Rbrace(char),
    Function,
    Let,
	Var,
    True,
    False,
    If,
    Else,
    Return,
    Minus(char),
    Bang(char),
    Asterisk(char),
    Slash(char),
    Lt(char),
    Gt(char)
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
	let identifier: String = ident.into_iter().collect();
	match &identifier[..] {
		"if" => Ok(Token::If),
		"else" => Ok(Token::Else),
		"fn" => Ok(Token::Function),
		"let" => Ok(Token::Let),
		"var" => Ok(Token::Var),
		"return" => Ok(Token::Return),
		"true" => Ok(Token::Return),
		"false" => Ok(Token::False),
		_ => Err(String::from("Not a keyword"))
	}
}