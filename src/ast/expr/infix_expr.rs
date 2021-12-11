use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    types::Type,
};

pub struct InfixExpr {
    pub operator: String,
    pub left: Option<Rc<dyn Expression>>,
    pub right: Option<Rc<dyn Expression>>,
    pub typ: Option<Type>,
}

impl InfixExpr {
    pub fn new(operator: String) -> Self {
        Self {
            operator,
            right: None,
            left: None,
            typ: None,
        }
    }
}

impl Node for InfixExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for InfixExpr {
    fn get_type(&self) -> Type {
        if let Some(typ) = self.typ {
            typ
        } else {
            Type::Unknown
        }
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = Some(typ)
    }
}

impl Display for InfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = String::new();
        prefix.push('(');
        prefix.push_str(&if let Some(ref left) = self.left {
            left.to_string()
        } else {
            "".to_string()
        });
        prefix.push_str(&format!(" {} ", &self.operator));
        prefix.push_str(&if let Some(ref right) = self.right {
            right.to_string()
        } else {
            "".to_string()
        });
        prefix.push(')');
        write!(f, "{}", prefix)
    }
}

