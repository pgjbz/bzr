use std::fmt::Display;

pub use super::node::Node;

pub trait Expression: Node {
    fn expression(&self);
}

impl Display for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}