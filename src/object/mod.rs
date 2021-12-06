use std::{any::Any, fmt::Display};

use crate::ast::types::Type;

pub mod boolean;
pub mod integer;
pub mod null;
pub mod ret;
pub mod string;

pub trait Object: Display {
    fn get_type(&self) -> Type;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
