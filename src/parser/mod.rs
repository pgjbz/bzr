mod errors;

use std::{mem, rc::Rc};

use crate::{
    ast::{
        expr::{bool_expr::BoolExpr, int_expr::IntExpr, str_expr::StrExpr},
        expression::Expression,
        identifier::Identifier,
        program::Program,
        statement::Statement,
        stmt::{let_stmt::Let, return_stmt::Return, var_stmt::Var},
        types::Type,
    },
    lexer::{token::Token, Lexer},
};

use self::errors::ParseError;

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
        'parse: loop {
            match self.parse_statement() {
                Ok(sts) => statements.push(sts),
                Err(e) => match e {
                    ParseError::Eof => break 'parse,
                    ParseError::Unknown => self.next_token(),
                    ParseError::Message(msg) => {
                        self.next_token();
                        self.errors.push(msg)
                    }
                },
            }
        }
        Program::new(statements, self.errors)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.current_token.as_ref() {
            Token::Let(_) => self.parse_let_var(true),
            Token::Var(_) => self.parse_let_var(false),
            Token::Return(_) => self.parse_return(),
            Token::EOF(_) => Err(ParseError::Eof),
            _ => Err(ParseError::Unknown),
        }
    }

    fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_var(&mut self, is_let: bool) -> Result<Box<dyn Statement>, ParseError> {
        self.expected_peek(Token::Ident(None, None))?;
        match self.extract_variables_fields() {
            Ok((identifier, typ, val)) => {
                self.print_error_str_empty(&val);
                match Self::parse_expression(self, Precedence::Lowest, val, &typ) {
                    Ok(expression) => {
                        if is_let {
                            Ok(Let::new(Token::Let(None), typ, identifier, expression))
                        } else {
                            Ok(Var::new(Token::Var(None), typ, identifier, expression))
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_return(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let ret = Return::new(None);
        //TODO: we're skipping expression until found a semicolon
        while !self._current_token_is(Token::Semicolon(None)) {
            self.next_token();
            if self._current_token_is(Token::EOF(None)) {
                let msg = format!("expected semicolon, got {}", self.current_token);
                return Err(ParseError::Message(msg));
            }
        }
        Ok(Box::new(ret))
    }

    //TODO: purge this method, its initial let test
    fn extract_variables_fields(&mut self) -> Result<(Identifier, Type, String), ParseError> {
        let identifier = match self.current_token.as_ref() {
            Token::Ident(identifier, _) => identifier,
            tok => {
                let msg = format!("Expected identifier, got {}", tok);
                return Err(ParseError::Message(msg));
            }
        };

        let identifier = Identifier::new(Some(Rc::clone(identifier.as_ref().unwrap())));

        let typ: Type;
        let val: String;
        //TODO: Use methods to get type to improve this método type validation
        if self.has_type() {
            self.next_token();
            typ = self.extract_type();
            self.expected_peek(Token::Assign(None))?;
            self.next_token();
            val = self.extract_value(&typ);
        } else {
            self.expected_peek(Token::Assign(None))?;
            self.next_token();
            let (t, v) = self.extract_value_and_type();
            typ = t;
            val = v;
        }
        Ok((identifier, typ, val))
    }

    fn print_error_str_empty(&mut self, str: &str) {
        if str.is_empty() {
            let tok = self.current_token.as_ref();
            self.errors.push(format!("expected value got {}", tok))
        }
    }

    fn has_type(&mut self) -> bool {
        self.peek_token_is(&Token::Int(None))
            || self.peek_token_is(&Token::Str(None))
            || self.peek_token_is(&Token::Bool(None))
    }

    fn extract_type(&self) -> Type {
        match *self.current_token {
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
                self.errors
                    .push(format!("expected {}, got {}", typ, self.current_token));
                String::from("")
            }
        }
    }

    fn extract_value_and_type(&mut self) -> (Type, String) {
        let typ: Type;
        let val = match self.current_token.as_ref() {
            Token::True(_) => {
                typ = Type::Bool;
                "true".to_string()
            }
            Token::False(_) => {
                typ = Type::Bool;
                "false".to_string()
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
                self.errors.push(format!(
                    "expected a value, got error {}",
                    self.current_token
                ));
                String::from("err")
            }
        };
        (typ, val)
    }

    //TODO: improve parse expression to really parse expression
    fn parse_expression(
        &mut self,
        precedence: Precedence,
        val: String,
        typ: &Type,
    ) -> Result<Box<dyn Expression>, ParseError> {
        match precedence {
            Precedence::Lowest => {
                self.expected_peek(Token::Semicolon(None))?;
                self.next_token();
                match typ {
                    Type::Int => {
                        if let Ok(value) = val.parse() {
                            Ok(Box::new(IntExpr::new(value)))
                        } else {
                            let msg = format!(
                                "error on parse token value {} {}",
                                val, self.current_token
                            );
                            Err(ParseError::Message(msg))
                        }
                    }
                    Type::String => Ok(Box::new(StrExpr::new(val))),
                    Type::Bool => {
                        if let Ok(value) = val.parse() {
                            Ok(Box::new(BoolExpr::new(value)))
                        } else {
                            let msg = format!(
                                "Error on parse token value {} {}",
                                val, self.current_token
                            );
                            Err(ParseError::Message(msg))
                        }
                    }
                    Type::Function => {
                        todo!()
                    }
                    Type::Unknown => {
                        let msg = "Unknown type".to_string();
                        Err(ParseError::Message(msg))
                    }
                }
            }
        }
    }

    fn expected_peek(&mut self, token: Token) -> Result<(), ParseError> {
        if self.peek_token_is(&token) {
            self.next_token();
            Ok(())
        } else {
            let msg = format!("expected {}, got {}", token, self.peek_token);
            Err(ParseError::Message(msg))
        }
    }

    fn type_is_equal(token_compare: &Type, token: &Type) -> bool {
        mem::discriminant(token_compare) == mem::discriminant(token)
    }

    fn peek_token_is(&mut self, token: &Token) -> bool {
        mem::discriminant(&*self.peek_token) == mem::discriminant(token)
    }

    fn _current_token_is(&mut self, token: Token) -> bool {
        mem::discriminant(&*self.current_token) == mem::discriminant(&token)
    }
}
