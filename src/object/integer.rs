use std::any::Any;

use crate::ast::types::Type;

use super::Object;

pub struct Integer {
    pub val: i64
}

impl Object for Integer {
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
