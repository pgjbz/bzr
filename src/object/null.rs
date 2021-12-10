use std::{any::Any, fmt::Display};

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

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}
