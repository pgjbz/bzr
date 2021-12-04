use std::any::Any;

use crate::ast::types::Type;

use super::Object;

pub struct Null;

impl Object for Null {
    fn get_type(&self) -> Type {
        Type::Unknown
    }

    fn inspect(&self) -> String {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
