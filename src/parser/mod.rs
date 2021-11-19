use std::{mem, rc::Rc};

use crate::{
    ast::{
        expr::{boolexpr::BoolExpr, intexpr::IntExpr, strexpr::StrExpr},
        expression::Expression,
        identifier::Identifier,
        program::Program,
        statement::Statement,
        stmt::{letstmt::Let, varstmt::Var},
        types::Type,
    },
    lexer::{token::Token, Lexer},
};

enum Precedence {
    Lowest,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Rc<Token>,
    peek_token: Rc<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
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

    fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_sts(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expected_peek(Token::Ident(None, None), true) {
            return None;
        }

        if let Some((identifier, typ, val)) = self.extract_variables_fields() {
            self.print_error_str_empty(&val);
            if let Ok(expression) = Self::parse_expression(self, Precedence::Lowest, val, &typ) {
                Some(Let::new(Token::Let(None), typ, identifier, expression))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_var_sts(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expected_peek(Token::Ident(None, None), true) {
            return None;
        }
        if let Some((identifier, typ, val)) = self.extract_variables_fields() {
            self.print_error_str_empty(&val);
            if let Ok(expression) = Self::parse_expression(self, Precedence::Lowest, val, &typ) {
                Some(Var::new(Token::Var(None), typ, identifier, expression))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn extract_variables_fields(&mut self) -> Option<(Identifier, Type, String)> {
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
        Some((identifier, typ, val))
    }

    fn print_error_str_empty(&mut self, str: &str) {
        if str.is_empty() {
            let tok = self.current_token.as_ref();
            self.errors
                .push(format!("Error expected value got {}", tok))
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
    ) -> Result<Box<dyn Expression>, String> {
        match precedence {
            Precedence::Lowest => {
                if self.expected_peek(Token::Semicolon(None), true) {
                    self.next_token();
                    match typ {
                        Type::Int => {
                            if let Ok(value) = val.parse() {
                                Ok(Box::new(IntExpr::new(value)))
                            } else {
                                Err(format!(
                                    "Error on parse token value {} {}",
                                    val, self.current_token
                                ))
                            }
                        }
                        Type::String => Ok(Box::new(StrExpr::new(val))),
                        Type::Bool => {
                            if let Ok(value) = val.parse() {
                                Ok(Box::new(BoolExpr::new(value)))
                            } else {
                                Err(format!(
                                    "Error on parse token value {} {}",
                                    val, self.current_token
                                ))
                            }
                        }
                        Type::Unknown => Err("Unknown type".to_string()),
                    }
                } else {
                    Err(format!("Error on parse token value {}", self.current_token))
                }
            }
        }
    }

    fn expected_peek(&mut self, token: Token, register_error: bool) -> bool {
        if self.peek_token_is(&token) {
            self.next_token();
            return true;
        }
        if register_error {
            self.peek_error(&token);
        }
        false
    }

    fn peek_error(&mut self, token: &Token) {
        let msg = format!("expected {}, got {}", token, self.peek_token);
        self.errors.push(msg);
    }

    fn type_is_equal(token_compare: &Type, token: &Type) -> bool {
        mem::discriminant(token_compare) == mem::discriminant(token)
    }

    fn peek_token_is(&mut self, token: &Token) -> bool {
        mem::discriminant(&*self.peek_token) == mem::discriminant(token)
    }

    fn current_token_is(&mut self, token: Token) -> bool {
        mem::discriminant(&*self.current_token) == mem::discriminant(&token)
    }
}
