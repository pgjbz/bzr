use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        stmt::block_stmt::BlockStatement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct FunctionExpr {
    pub token: Rc<Token>,
    pub parameters: Vec<Rc<dyn Expression>>,
    pub name: Rc<dyn Expression>,
    pub body: Option<Rc<BlockStatement>>,
    pub ret_typ: Type,
}

impl FunctionExpr {
    pub fn new(token: Rc<Token>, name: Rc<dyn Expression>) -> Self {
        Self {
            token,
            parameters: vec![],
            body: None,
            name,
            ret_typ: Type::Unknown,
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
            parameters.push(format!("{}", param))
        }

        buffer.push_str(&format!("fn {} (", self.name));
        buffer.push_str(&parameters.join(","));
        buffer.push_str(") ");
        buffer.push_str(&match self.ret_typ {
            Type::Bool | Type::Int | Type::String => format!("{} ", self.ret_typ),
            _ => "".to_string(),
        });
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
