use std::any::Any;

use crate::ast::types::Type;

use super::Object;

pub struct Str {
    pub val: String
}

impl Object for Str {
    fn get_type(&self) -> Type {
        Type::Int
    }

    fn inspect(&self) -> String {
        self.val.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
