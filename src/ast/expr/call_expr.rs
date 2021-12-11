use std::{fmt::Display, rc::Rc};

use crate::ast::expression::{Expression, Node};

pub struct CallExpr {
    pub function: Rc<dyn Expression>,
    pub arguments: Vec<Rc<dyn Expression>>,
}

impl CallExpr {
    pub fn new(function: Rc<dyn Expression>) -> Self {
        Self {
            function,
            arguments: vec![],
        }
    }
}

impl Node for CallExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for CallExpr {
    fn get_type(&self) -> crate::ast::types::Type {
        self.function.get_type()
    }
}

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        let mut arguments = Vec::<String>::with_capacity(5);
        for arg in self.arguments.iter() {
            arguments.push(arg.to_string());
        }

        buffer.push_str(&self.function.to_string());
        buffer.push('(');
        buffer.push_str(&arguments.join(","));
        buffer.push(')');
        write!(f, "{}", buffer)
    }
}
