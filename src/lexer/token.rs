#[derive(PartialEq, Debug)]
pub enum Token {
	Illegal(usize),
    EOF(usize),
    Ident(String, usize),
    Number(String, usize),
    Assign(char, usize),
    Plus(char, usize),
    Comma(char, usize),
    Semicolon(char, usize),
    Lparen(char, usize),
    Rparen(char, usize),
    Lbrace(char, usize),
    Rbrace(char, usize),
	String(String, usize),
    Function(usize),
    Let(usize),
	Var(usize),
    True(usize),
    False(usize),
    If(usize),
    Else(usize),
    Return(usize),
	Int(usize),
	Str(usize),
    Minus(char, usize),
    Bang(char, usize),
    Asterisk(char, usize),
    Slash(char, usize),
    Lt(char, usize),
    Gt(char, usize)
}

pub fn get_keyword_token(ident: &str, position: usize) -> Result<Token, String> {
	let identifier: String = String::from(ident);
	match &identifier[..] {
		"if" => Ok(Token::If(position),),
		"else" => Ok(Token::Else(position)),
		"fn" => Ok(Token::Function(position)),
		"let" => Ok(Token::Let(position)),
		"var" => Ok(Token::Var(position)),
		"ret" => Ok(Token::Return(position)),
		"true" => Ok(Token::Return(position)),
		"false" => Ok(Token::False(position)),
		"int" => Ok(Token::Int(position)),
		"str" => Ok(Token::Str(position)),
		_ => Err(String::from("Not a keyword"))
	}
}