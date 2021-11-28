use std::fmt::Display;

pub use super::node::Node;
use super::types::Type;

pub trait Expression: Node {
    fn expression(&self);
    fn get_type(&self) -> Type;
}

impl Display for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
