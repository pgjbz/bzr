use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    stmt::block_stmt::BlockStatement,
    types::Type,
};

pub struct WhileExpr {
    pub condition: Rc<dyn Expression>,
    pub consequence: Option<Rc<BlockStatement>>,
}

impl WhileExpr {
    pub fn new(condition: Rc<dyn Expression>) -> Self {
        Self {
            condition,
            consequence: None,
        }
    }
}

impl Node for WhileExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for WhileExpr {
    fn get_type(&self) -> Type {
        Type::Expression
    }
}

impl Display for WhileExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("while ");
        buffer.push_str(&format!("{} ", self.condition));
        buffer.push_str(&if let Some(ref consequence) = self.consequence {
            consequence.to_string()
        } else {
            "".to_string()
        });
        write!(f, "{}", buffer)
    }
}

