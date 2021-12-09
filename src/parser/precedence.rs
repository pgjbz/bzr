use std::cmp::Ordering;

use crate::lexer::token::Token;

#[derive(Clone, Copy, Debug)]
pub(super) enum Precedence {
    Lowest = 1,
    AndOr = 2,
    Equals = 3,
    LessGreater = 4,
    Sum = 5,
    Product = 6,
    Prefix = 7,
    Call = 8,
    Index = 9,
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
        Token::Or(_) | Token::And(_) => Precedence::AndOr,
        Token::Lt(_) | Token::Gt(_) | Token::Lte(_) | Token::Gte(_) | Token::Assign(_) => {
            Precedence::LessGreater
        }
        Token::Plus(_) | Token::Minus(_) => Precedence::Sum,
        Token::Slash(_) | Token::Asterisk(_) => Precedence::Product,
        Token::LParen(_) => Precedence::Call,
        Token::LSqBracket(_) => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
