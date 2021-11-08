#[derive(PartialEq, Debug)]
pub enum Token {
	Illegal(Location),
    EOF(Location),
    Ident(String, Location),
    Number(String, Location),
    Assign(Location),
    Plus(Location),
    Comma(Location),
    Semicolon(Location),
    Lparen(Location),
    Rparen(Location),
    Lbrace(Location),
    Rbrace(Location),
	String(String, Location),
    Function(Location),
    Let(Location),
	Var(Location),
    True(Location),
    False(Location),
    If(Location),
    Else(Location),
    Return(Location),
	Int(Location),
	Str(Location),
	Bool(Location),
    Minus(Location),
    Bang(Location),
    Asterisk(Location),
    Slash(Location),
    Lt(Location),
    Gt(Location)
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location {
	position: usize,
	line: usize,
	filename: &'static str
} 

impl Location {
	pub fn new(position: usize, line: usize, filename: &'static str) -> Self {
		Self {
			position,
			line,
			filename
		}
	}
}

impl Token {
	pub fn get_keyword_token(ident: &str, location: Location) -> Result<Token, String> {
		let identifier: String = String::from(ident);
		match &identifier[..] {
			"if" => Ok(Token::If(location),),
			"else" => Ok(Token::Else(location)),
			"fn" => Ok(Token::Function(location)),
			"let" => Ok(Token::Let(location)),
			"var" => Ok(Token::Var(location)),
			"ret" => Ok(Token::Return(location)),
			"true" => Ok(Token::True(location)),
			"false" => Ok(Token::False(location)),
			"int" => Ok(Token::Int(location)),
			"str" => Ok(Token::Str(location)),
			"bool" => Ok(Token::Bool(location)),
			_ => Err(String::from("Not a keyword"))
		}
	}
}