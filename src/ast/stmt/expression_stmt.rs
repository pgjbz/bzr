use crate::{ast::{expression::{Expression, Node}, types::Type}, lexer::token::Token};

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
    pub typ: Type,
    pub token: Token
}

impl ExpressionStatement {

}

impl Node for ExpressionStatement {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        todo!()
    }
}

impl Expression for ExpressionStatement {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        self.typ
    }
}