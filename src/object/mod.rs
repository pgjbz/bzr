use std::any::Any;

use crate::ast::types::Type;

pub mod boolean;
pub mod integer;
pub mod null;
pub mod string;

pub trait Object {
    fn get_type(&self) -> Type;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
