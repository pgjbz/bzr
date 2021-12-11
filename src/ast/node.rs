use std::{any::Any, fmt::Display};

pub trait Node: Display {
    fn as_any(&self) -> &dyn Any;
}
