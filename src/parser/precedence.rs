use std::cmp::Ordering;

use crate::lexer::token::Token;

#[derive(Clone, Copy)]
pub(super) enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

impl PartialEq for Precedence {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (cur, other) if cur == other => Some(Ordering::Equal),
            (cur, other) if (*cur as usize) > (*other as usize) => Some(Ordering::Greater),
            (cur, other) if (*cur as usize) < (*other as usize) => Some(Ordering::Less),
            _ => Some(Ordering::Less),
        }
    }
}

pub(super) fn get_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq(_) | Token::Diff(_) => Precedence::Equals,
        Token::Lt(_)
        | Token::Gt(_)
        | Token::Lte(_)
        | Token::Gte(_)
        | Token::And(_)
        | Token::Assign(_)
        | Token::Or(_) => Precedence::LessGreater,
        Token::Plus(_) | Token::Minus(_)  => Precedence::Sum,
        Token::Slash(_) | Token::Asterisk(_) => Precedence::Product,
        Token::LParen(_) => Precedence::Call,
        _ => Precedence::Lowest,
    }
}
