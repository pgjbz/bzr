use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::Expression, identifier::Identifier, stmt::block_stmt::BlockStatement, types::Type,
};

use super::{environment::Environment, Object};

pub struct Function {
    pub parameters: Vec<Identifier>,
    pub name: Rc<dyn Expression>,
    pub body: BlockStatement,
    pub env: Rc<Environment>,
}

impl Function {
    pub fn new(
        parameters: Vec<Identifier>,
        name: Rc<dyn Expression>,
        body: BlockStatement,
        env: Rc<Environment>,
    ) -> Self {
        Self {
            parameters,
            name,
            body,
            env,
        }
    }
}

impl Object for Function {
    fn get_type(&self) -> crate::ast::types::Type {
        Type::Function
    }

    fn inspect(&self) -> String {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        let mut args = Vec::with_capacity(5);
        for param in self.parameters.iter() {
            args.push(param.to_string())
        }
        buffer.push_str("fn (");
        buffer.push_str(&args.join(","));
        buffer.push_str(") {\n");
        buffer.push_str(&self.body.to_string());
        buffer.push_str("\n}");
        write!(f, "{}", buffer)
    }
}
