use crate::{
    ast::{
        expression::Expression, identifier::Identifier, node::Node, statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct Let<'a> {
    pub token: Token<'a>,
    pub typ: Type,
    pub name: Identifier<'a>,
    pub value: Box<dyn Expression<'a>>,
}

impl<'a> Let<'a> {
    pub fn new(
        token: Token<'a>,
        typ: Type,
        name: Identifier<'a>,
        value: Box<dyn Expression<'a>>,
    ) -> Box<Self> {
        Box::new({
            Self {
                token,
                typ,
                name,
                value,
            }
        })
    }
}

impl<'a> Node<'a> for Let<'a> {
    fn literal(&self) -> &'a str {
        "let"
    }
}

impl<'a> Statement<'a> for Let<'a> {
    fn statement(&self) {
        todo!()
    }
}
