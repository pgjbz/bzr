use std::any::Any;

use crate::ast::types::Type;

pub mod integer;
pub mod boolean;
pub mod string;
pub mod null;

pub trait Object {
    fn get_type(&self) -> Type;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}