use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::ast::{expression::Expression, stmt::block_stmt::BlockStatement, types::Type};

use super::{environment::Environment, Object};

pub struct Function {
    pub parameters: Vec<Rc<dyn Expression>>,
    pub name: Rc<dyn Expression>,
    pub body: Option<Rc<BlockStatement>>,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<Rc<dyn Expression>>,
        name: Rc<dyn Expression>,
        body: Option<Rc<BlockStatement>>,
        env: Rc<RefCell<Environment>>,
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
        buffer.push_str(&if let Some(ref body) = self.body {
            body.to_string()
        } else {
            String::new()
        });
        buffer.push_str("\n}");
        write!(f, "{}", buffer)
    }
}
