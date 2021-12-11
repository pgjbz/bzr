use std::fmt::Display;
use std::rc::Rc;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct ArrayExpr {
    pub value: Vec<Rc<dyn Expression>>,
}

impl ArrayExpr {
    pub fn new(value: Vec<Rc<dyn Expression>>) -> Self {
        Self { value }
    }
}

impl Node for ArrayExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for ArrayExpr {
    fn get_type(&self) -> Type {
        Type::Array
    }
}

impl Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push('[');
        for (pos, val) in self.value.iter().enumerate() {
            if pos < 1 {
                buffer.push_str(&val.to_string())
            } else {
                buffer.push_str(&format!(", {}", val))
            }
        }
        buffer.push(']');
        write!(f, "{}", buffer)
    }
}
