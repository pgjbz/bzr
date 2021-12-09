use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::types::Type;

use super::Object;

pub type BuildInFn = fn(&[Rc<dyn Object>]) -> Rc<dyn Object>;

pub struct BuiltIn {
    pub function: BuildInFn,
}

impl BuiltIn {
    pub fn new(function: BuildInFn) -> Self {
        Self { function }
    }
}

impl Object for BuiltIn {
    fn get_type(&self) -> Type {
        Type::Function
    }

    fn inspect(&self) -> String {
        "build in function".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "build in function")
    }
}
