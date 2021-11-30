use std::{any::Any, fmt::Display};

pub trait Node: Display {
    fn literal(&self) -> Box<dyn Display>;
    fn as_any(&self) -> &dyn Any;
}
