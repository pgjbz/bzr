use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::types::Type;

use super::Object;

pub struct Array {
    pub elements: Vec<Rc<dyn Object>>,
}

impl Array {
    pub fn new(elements: Vec<Rc<dyn Object>>) -> Self {
        Self { elements }
    }
}

impl Object for Array {
    fn get_type(&self) -> Type {
        Type::Array
    }

    fn inspect(&self) -> String {
        let mut buffer = String::new();
        buffer.push('[');
        for (idx, elem) in self.elements.iter().enumerate() {
            if idx == 0 {
                buffer.push_str(&elem.to_string())
            } else {
                buffer.push_str(&format!(", {}", elem))
            }
        }
        buffer.push(']');
        buffer
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}
