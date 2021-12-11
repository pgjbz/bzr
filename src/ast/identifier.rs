use crate::ast::expression::Expression;
use crate::ast::node::Node;
use crate::ast::types::Type;
use std::{fmt::Display, rc::Rc};

pub struct Identifier {
    pub value: Rc<String>,
    pub typ: Option<Type>,
}

impl Identifier {
    pub fn new(value: Rc<String>) -> Self {
        Self { value, typ: None }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref())
    }
}

impl Node for Identifier {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Identifier {
    fn get_type(&self) -> Type {
        Type::Unknown
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = Some(typ);
    }
}
