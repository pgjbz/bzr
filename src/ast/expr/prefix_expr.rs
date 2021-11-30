use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        types::Type,
    },
    lexer::token::Token,
};

pub struct PrefixExpr {
    pub token: Rc<Token>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl PrefixExpr {
    pub fn new(token: Rc<Token>, operator: String) -> Self {
        Self {
            token,
            operator,
            right: None,
        }
    }
}

impl Node for PrefixExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.token.to_string())
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
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Prefix
    }
}