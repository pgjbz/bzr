use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        identifier::Identifier,
        stmt::block_stmt::BlockStatement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct FunctionExpr {
    pub token: Rc<Token>,
    pub parameters: Vec<Identifier>,
    pub name: Box<dyn Expression>,
    pub body: Option<Box<BlockStatement>>,
}

impl FunctionExpr {
    pub fn new(token: Rc<Token>, name: Box<dyn Expression>) -> Self {
        Self {
            token,
            parameters: vec![],
            body: None,
            name,
        }
    }
}

impl Node for FunctionExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new("fn".to_string())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for FunctionExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        let mut parameters = Vec::new();
        for param in self.parameters.iter() {
            parameters.push(param.to_string())
        }

        buffer.push_str(&format!("fn {} (", self.name));
        buffer.push_str(&parameters.join(","));
        buffer.push(')');
        buffer.push_str(&if let Some(ref body) = self.body {
            body.to_string()
        } else {
            "".to_string()
        });
        write!(f, "{}", buffer)
    }
}

impl Expression for FunctionExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Function
    }
}
