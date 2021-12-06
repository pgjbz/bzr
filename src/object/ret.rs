use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::types::Type;

use super::Object;

pub struct Ret {
    pub val: Rc<dyn Object>,
}

impl Ret {
    pub fn new(val: Rc<dyn Object>) -> Self {
        Self { val }
    }
}

impl Object for Ret {
    fn get_type(&self) -> Type {
        self.val.get_type()
    }

    fn inspect(&self) -> String {
        self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Ret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
