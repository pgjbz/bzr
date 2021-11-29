use crate::parser::Parser;

use self::expression::Expression;

pub mod expr;
pub mod expression;
pub mod identifier;
pub mod node;
pub mod program;
pub mod statement;
pub mod stmt;
pub mod types;

pub type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
pub type InfixParseFn = fn(&mut Parser, dyn Expression) -> Box<dyn Expression>;