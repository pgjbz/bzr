use super::node::Node;

pub trait Statement<'a>: Node<'a> {
    fn statement(&self);
}
