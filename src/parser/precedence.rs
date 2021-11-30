use std::cmp::Ordering;

use crate::lexer::token::Token;

#[derive(Clone, Copy)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    _Call = 7,
}

impl PartialEq for Precedence {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        let cur = *self as usize;
        let oth = *other as usize;

        if cur > oth {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

pub fn get_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq(_) | Token::Diff(_) => Precedence::Equals,
        Token::Lt(_) | Token::Gt(_) | Token::Lte(_) | Token::Gte(_) => Precedence::LessGreater,
        Token::Plus(_) | Token::Minus(_) => Precedence::Sum,
        Token::Slash(_) | Token::Asterisk(_) => Precedence::Product,
        _ => Precedence::Lowest,
    }
}
