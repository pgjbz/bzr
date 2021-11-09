use std::mem;

use crate::{lexer::{Lexer, token::Token}};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub current_token: Box<Token<'a>>,
    pub peek_token: Box<Token<'a>>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![]
        }
    }

    pub fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_let_sts(&mut self) {

    }

    pub fn expected_peek(&mut self, token: Token<'a>) -> bool {
        if self.peek_token_is(Token::Ident(None, None)) {
            self.next_token();
            return true;
        } 
        self.peek_error(token);
        return false;
    }

    pub fn peek_error(&mut self, token: Token<'a>) {
        let msg = format!("expected {}, got {}", 
        token, 
        self.peek_token);
        self.errors.push(msg);
    }

    pub fn peek_token_is(&self, token: Token) -> bool {
        mem::discriminant(&*self.peek_token) == mem::discriminant(&token)
    }

    pub fn current_token_is(&self, token: Token) -> bool {
        mem::discriminant(&*self.current_token) == mem::discriminant(&token)
    }
}