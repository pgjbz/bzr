use super::node::Node;

pub trait Statement: Node {
    fn statement(&self);
}
