use std::any::Any;

use crate::ast::types::Type;

use super::Object;

pub struct Boolean {
    pub val: bool
}

impl Object for Boolean {
    fn get_type(&self) -> Type {
        Type::Int
    }

    fn inspect(&self) -> String {
        self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
