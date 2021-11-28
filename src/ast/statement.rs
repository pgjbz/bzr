use crate::lexer::token::Token;

use super::node::Node;

pub trait Statement: Node {
    fn statement(&self);
    fn get_statament_token(&self) -> Token;
}
