use std::{any::Any, cell::RefCell};

use crate::ast::types::Type;

use super::Object;

pub struct Boolean {
    pub val: RefCell<bool>,
}

impl Boolean {
    pub fn new(val: bool) -> Self {
        Self {
            val: RefCell::new(val),
        }
    }
}

impl Object for Boolean {
    fn get_type(&self) -> Type {
        Type::Int
    }

    fn inspect(&self) -> String {
        self.val.take().to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
