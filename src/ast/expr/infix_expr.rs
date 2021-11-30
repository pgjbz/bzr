use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        types::Type,
    },
    lexer::token::Token,
};

pub struct InfixExpr {
    pub token: Rc<Token>,
    pub operator: String,
    pub left: Option<Box<dyn Expression>>,
    pub right: Option<Box<dyn Expression>>,
    pub typ: Option<Type>,
}

impl InfixExpr {
    pub fn new(token: Rc<Token>, operator: String) -> Self {
        Self {
            token,
            operator,
            right: None,
            left: None,
            typ: None,
        }
    }
}

impl Node for InfixExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.token.to_string())
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

impl Expression for InfixExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        self.typ.unwrap()
    }
}
