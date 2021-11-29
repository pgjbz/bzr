pub use super::node::Node;
use super::types::Type;

pub trait Expression: Node {
    fn expression(&self);
    fn get_type(&self) -> Type;
}
