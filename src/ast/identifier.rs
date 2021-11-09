use crate::lexer::token::Token;

pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(token: Token<'a>, value: &'a str) -> Box<Self> {
        Box::new(Self { token, value })
    }
}
