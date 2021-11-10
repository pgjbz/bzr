use std::{cell::RefCell, mem, rc::Rc};

use crate::{ast::{expression::Expression, exs::{boolex::BoolEx, intex::IntEx, strex::StrEx}, identifier::Identifier, node::Node, program::Program, statement::Statement, sts::{letsts::Let, varsts::Var}, types::Type}, lexer::{token::Token, Lexer}};

enum Precedence {
    Lowest,
}

pub struct Parser {
    pub lexer: RefCell<Lexer>,
    pub current_token: Rc<Token>,
    pub peek_token: Rc<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: RefCell<Lexer>) -> Self {
        let current_token = lexer.borrow_mut().next_token();
        let peek_token = lexer.borrow_mut().next_token();
        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    pub fn parse_program(mut self) -> Box<Program> {
        let mut statements = vec![];
        while !self.current_token_is(Token::EOF(None)) {
            if let Some(sts) = self.parse_statement() {
                statements.push(sts)
            } else {
                self.next_token();
            }
        }
        let mut errors = vec![];
        for error in &mut self.errors {
            errors.push(error.clone());
        }
        Program::new(statements, errors)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.as_ref() {
            Token::Let(_) => self.parse_let_sts(),
            Token::Var(_) => self.parse_var_sts(),
            Token::EOF(_) => None,
            _ => {
                self.current_token = Rc::new(Token::EOF(None));
                None
            }
        }
    }

    pub fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.borrow_mut().next_token();
    }

    pub fn parse_let_sts(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expected_peek(Token::Ident(None, None), true) {
            return None;
        }

        let identifier = match self.current_token.as_ref() {
            Token::Ident(identifier, _) => identifier,
            _ => return None,
        };

        let identifier = Identifier::new(Some(Rc::clone(identifier.as_ref().unwrap())));

        let typ: Type;
        let val: String;
        if self.has_type() {
            typ = self.extract_type();
            if !self.expected_peek(Token::Assign(None), true) {
                return None;
            }
            self.next_token();
            val = self.extract_value(&typ);
        } else {
            if !self.expected_peek(Token::Assign(None), true) {
                return None;
            }
            self.next_token();
            let (t, v) = self.extract_value_and_type();
            typ = t;
            val = v;
        }
        self.print_error_str_empty(&val);
        if let Some(expression) = Self::parse_expression(self, Precedence::Lowest, val, &typ) {
            Some(Let::new(Token::Let(None), typ, identifier, expression))
        } else {
            None
        }
    }

    pub fn parse_var_sts(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expected_peek(Token::Ident(None, None), true) {
            return None;
        }

        let identifier = match self.current_token.as_ref() {
            Token::Ident(identifier, _) => identifier,
            _ => return None,
        };

        let identifier = Identifier::new(Some(Rc::clone(identifier.as_ref().unwrap())));

        let typ: Type;
        let val: String;
        if self.has_type() {
            typ = self.extract_type();
            if !self.expected_peek(Token::Assign(None), true) {
                return None;
            }
            self.next_token();
            val = self.extract_value(&typ);
        } else {
            if !self.expected_peek(Token::Assign(None), true) {
                return None;
            }
            self.next_token();
            let (t, v) = self.extract_value_and_type();
            typ = t;
            val = v;
        }
        self.print_error_str_empty(&val);
        if let Some(expression) = Self::parse_expression(self, Precedence::Lowest, val, &typ) {
            Some(Var::new(Token::Var(None), typ, identifier, expression))
        } else {
            None
        }
    }

    fn print_error_str_empty(&mut self, str: &str) {
        if str.is_empty() {
            for error in &mut self.errors {
                println!("{}", error);
            }
            panic!("error");
        }
    }

    fn has_type(&mut self) -> bool {
        self.expected_peek(Token::Int(None), false)
            || self.expected_peek(Token::Str(None), false)
            || self.expected_peek(Token::Bool(None), false)
    }

    fn extract_type(&self) -> Type {
        match self.current_token.as_ref() {
            Token::Int(_) => Type::Int,
            Token::Str(_) => Type::String,
            Token::Bool(_) => Type::Bool,
            _ => Type::Unknown,
        }
    }

    fn extract_value(&mut self, typ: &Type) -> String {
        match self.current_token.as_ref() {
            Token::True(_) if Self::type_is_equal(typ, &Type::Bool) => String::from("true"),
            Token::False(_) if Self::type_is_equal(typ, &Type::Bool) => String::from("false"),
            Token::Number(val, _) if Self::type_is_equal(typ, &Type::Int) => {
                val.as_ref().unwrap().as_ref().clone()
            }
            Token::String(val, _) if Self::type_is_equal(typ, &Type::String) => {
                val.as_ref().unwrap().as_ref().clone()
            }
            _ => {
                self.errors.push(format!(
                    "Invalid type expected {}, got {}",
                    typ, self.current_token
                ));
                String::from("")
            }
        }
    }

    fn extract_value_and_type(&mut self) -> (Type, String) {
        let typ: Type;
        let val = match self.current_token.as_ref() {
            Token::True(_) | Token::False(_) => {
                typ = Type::Bool;
                format!("{}", typ)
            }
            Token::Number(val, _) => {
                typ = Type::Int;
                val.as_ref().unwrap().as_ref().clone()
            }
            Token::String(val, _) => {
                typ = Type::String;
                val.as_ref().unwrap().as_ref().clone()
            }
            _ => {
                typ = Type::Unknown;
                self.errors
                    .push(format!("expected a type, got error {}", self.current_token));
                String::from("")
            }
        };
        (typ, val)
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
        val: String,
        typ: &Type,
    ) -> Option<Box<dyn Expression>> {
        match precedence {
            Precedence::Lowest => {
                if self.expected_peek(Token::Semicolon(None), true) {
                    self.next_token();
                    match typ {
                        Type::Int => {
                            if let Ok(value) = val.parse() {
                                Some(Box::new(IntEx::new(value)))
                            } else {
                                None
                            }
                        },
                        Type::String => Some(Box::new(StrEx::new(val))),
                        Type::Bool => {
                            if let Ok(value) = val.parse() {
                                Some(Box::new(BoolEx::new(value)))
                            } else {
                                None
                            }
                        },
                        Type::Unknown => None
                    }
                } else {
                    None
                }
            }
        }
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
        let msg = format!("expected {}, got {}", token, self.peek_token);
        self.errors.push(msg);
    }

    pub fn type_is_equal(token_compare: &Type, token: &Type) -> bool {
        mem::discriminant(token_compare) == mem::discriminant(token)
    }

    pub fn peek_token_is(&mut self, token: &Token) -> bool {
        mem::discriminant(&*self.peek_token) == mem::discriminant(token)
    }

    pub fn current_token_is(&mut self, token: Token) -> bool {
        mem::discriminant(&*self.current_token) == mem::discriminant(&token)
    }
}

impl Node for String {
    fn literal(&self) -> String {
        self.to_string()
    }
}

impl Expression for String {
    fn expression(&self) {
        todo!()
    }
}
