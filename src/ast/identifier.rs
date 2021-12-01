use crate::ast::node::Node;
use crate::ast::types::Type;
use crate::{ast::expression::Expression, lexer::token::Token};
use std::{fmt::Display, rc::Rc};

pub struct Identifier {
    pub token: Rc<Token>,
    pub value: Rc<String>,
    pub typ: Option<Type>,
}

impl Identifier {
    pub fn new(value: Rc<String>, token: Rc<Token>) -> Self {
        Self {
            value,
            token,
            typ: None,
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref())
    }
}

impl Node for Identifier {
    fn literal(&self) -> std::boxed::Box<dyn Display> {
        Box::new("identifier".to_string())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Identifier {
    fn expression(&self) {
        todo!()
    }
    fn get_type(&self) -> Type {
        Type::Unknown
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = Some(typ);
    }
}
