pub use super::node::Node;
use super::types::Type;

pub trait Expression: Node {
    fn get_type(&self) -> Type;
    fn set_type(&mut self, _typ: Type) {}
}
