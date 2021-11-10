use std::{mem, rc::Rc};

use crate::{ast::{expression::Expression, identifier::Identifier, node::Node, statements::letsts::Let, types::Type}, lexer::{Lexer, token::Token}};

enum Precedence {
    Lowest = 0,
}

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Rc<Token>,
    pub peek_token: Rc<Token>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: &'a mut Lexer) -> Self {
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

    pub fn parse_let_sts(&mut self) -> Option<Box<Let>> {

        if !self.expected_peek(Token::Ident(None, None), true) {
            return None;
        }

        let identifier = 
            match self.current_token.as_ref() {
                Token::Ident(identifier, _) => identifier,
                _ => return None
            };

        let identifier = Identifier::new(Some(Rc::clone(identifier.as_ref().unwrap())));
        //let nome tipo = valor;
        //let nome = valor;

        let typ: Type;
        let val: String;
        if self.expected_peek(Token::Int(None), false)
            || self.expected_peek(Token::Str(None), false)
            || self.expected_peek(Token::Bool(None), false)
        {
            typ = match self.current_token.as_ref() {
                Token::Int(_) => Type::Int,
                Token::Str(_) => Type::String,
                Token::Bool(_) => Type::Bool,
                _ =>  Type::Unknown
            };
            if !self.expected_peek(Token::Assign(None), true) {
                return None;
            }
            self.next_token();
            println!("{:?}", self.current_token);
            val = match self.current_token.as_ref() {
                Token::True(_) if Self::type_is_equal(&typ, &Type::Bool) => String::from("true"),
                Token::False(_) if Self::type_is_equal(&typ, &Type::Bool) => String::from("false"),
                Token::Number(val, _) if Self::type_is_equal(&typ, &Type::Int)  => val.as_ref().unwrap().as_ref().clone(),
                Token::String(val, _) if Self::type_is_equal(&typ, &Type::String) => val.as_ref().unwrap().as_ref().clone(),
                _ => {
                    self.errors.push(format!("Invalid type expected {}, got {}", typ, self.current_token));
                    String::from("")
                }
            };
            if val.is_empty() {
                for error in &mut self.errors {
                    println!("{}", error);
                }
                panic!("INVALID TYPE");
            }
        } else {
            typ = Type::Bool;
            val = String::from("")
        }
        let expression = Self::parse_expression(Precedence::Lowest as isize, val);
        Some(Let::new(Token::Let(None), typ, identifier, expression))
        // None
    }

    fn parse_expression(_precedence: isize, val: String) -> Box<dyn Expression> {
        Box::new(val)
    }

    pub fn expected_peek(&mut self, token: Token, register_error: bool) -> bool {
        if self.peek_token_is(&token) {
            self.next_token();
            return true;
        } 
        if register_error {
            self.peek_error(&token);
        }
        false
    }

    pub fn peek_error(&mut self, token: &Token) {
        let msg = format!("expected {}, got {}", 
        token, 
        self.peek_token);
        self.errors.push(msg);
    }

    pub fn type_is_equal(token_compare: &Type, token: &Type) -> bool{
        mem::discriminant(token_compare) == mem::discriminant(token)
    }

    pub fn peek_token_is(&self, token: &Token) -> bool {
        mem::discriminant(&*self.peek_token) == mem::discriminant(token)
    }


    pub fn current_token_is(&self, token: Token) -> bool {
        mem::discriminant(&*self.current_token) == mem::discriminant(&token)
    }
}

impl Node for String {
    fn literal(&self) -> String {
        todo!()
    }
}

impl Expression for String {
    fn expression(&self) {
        todo!()
    }
}