pub use super::node::Node;

pub trait Expression<'a>: Node<'a> {
    fn expression(&self);
}
