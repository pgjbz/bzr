use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    types::Type,
};

pub struct PrefixExpr {
    pub operator: String,
    pub right: Option<Rc<dyn Expression>>,
    pub typ: Option<Type>,
}

impl PrefixExpr {
    pub fn new(operator: String) -> Self {
        Self {
            operator,
            right: None,
            typ: None,
        }
    }
}

impl Node for PrefixExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = String::new();
        prefix.push('(');
        prefix.push_str(&self.operator);

        prefix.push_str(&if let Some(ref right) = self.right {
            right.to_string()
        } else {
            "".to_string()
        });
        prefix.push(')');
        write!(f, "{}", prefix)
    }
}

impl Expression for PrefixExpr {
    fn get_type(&self) -> Type {
        Type::Prefix
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = Some(typ)
    }
}
