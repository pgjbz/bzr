pub mod errors;

use std::{mem, rc::Rc, collections::HashMap};

use crate::{ast::{InfixParseFn, PrefixParseFn, expr::{bool_expr::{BoolExpr}, int_expr::IntExpr, prefix_expr::PrefixExpr, str_expr::StrExpr}, expression::Expression, identifier::Identifier, program::Program, statement::Statement, stmt::{let_stmt::Let, return_stmt::Return, var_stmt::Var, expression_stmt::ExpressionStatement}, types::Type}, lexer::{token::Token, Lexer}};

use self::errors::ParseError;

enum Precedence {
    Lowest,
    _Equals,
    _LessGreater,
    _Sum,
    _Product,
    Prefix,
    _Call
}

pub struct Parser {
    lexer: Lexer,
    current_token: Rc<Token>,
    peek_token: Rc<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<Token, PrefixParseFn>,
    _infix_parse_fns: HashMap<Token, InfixParseFn>,
}

impl Parser {

    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut prefix_parse_fns: HashMap<Token, PrefixParseFn> = HashMap::new();
        prefix_parse_fns.insert(Token::Ident(None, None), Self::parse_identifier);
        prefix_parse_fns.insert(Token::Number(None, None), Self::parse_number_literal);
        prefix_parse_fns.insert(Token::Minus(None), Self::parse_prefix_expression);
        prefix_parse_fns.insert(Token::Bang(None), Self::parse_prefix_expression);
        prefix_parse_fns.insert(Token::True(None), Self::parse_bool_literal);
        prefix_parse_fns.insert(Token::False(None), Self::parse_bool_literal);
        prefix_parse_fns.insert(Token::String(None, None), Self::parse_string_literal);
        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parse_fns,
            _infix_parse_fns: HashMap::new(),
        }
    }

    pub fn parse_program(mut self) -> Box<Program> {
        let mut statements = vec![];
        'parse: loop {
            match self.parse_statement() {
                Ok(sts) => {
                    self.next_token();
                    statements.push(sts)
                },
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
            _ => self.parse_expression_statement()
        }
    }

    fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_var(&mut self, is_let: bool) -> Result<Box<dyn Statement>, ParseError> {
        let current_token = Rc::clone(&self.current_token);
        self.expected_peek(Token::Ident(None, None))?;
        let identifier: Box<dyn Expression> = Self::parse_identifier(self)?;
        let expression: Box<dyn Expression>;
        if self.has_type() {
            self.next_token();
            let declared_type = Some(self.current_token.to_type());
            self.expected_peek(Token::Assign(None))?;
            self.next_token();
            expression = if let Ok(expr) = self.parse_expression(Precedence::Lowest) {
                expr
            } else {
                let msg = format!("error on parse expression {}", current_token);
                return Err(ParseError::Message(msg))
            };
            if declared_type.unwrap() != expression.get_type() {
                let msg = format!("expected {}, got {}", declared_type.unwrap(), expression.get_type());
                return Err(ParseError::Message(msg))
            }
        } else {
            self.expected_peek(Token::Assign(None))?;
            self.next_token();
            expression = match  self.parse_expression(Precedence::Lowest) {
                Ok(expr) => expr,
                Err(e) => {
                    let error_msg = if let ParseError::Message(msg) = e {
                        msg
                    } else {
                        "".to_string()
                    };
                    let msg = format!("error on parse expression {}... {}", current_token, error_msg);
                    return Err(ParseError::Message(msg))
                }
            }
        }
        if is_let {
            Ok(Let::new(
                current_token,
                expression.get_type(),
                identifier,
                expression
            ))
        } else {
            Ok(Var::new(
                current_token,
                expression.get_type(),
                identifier,
                expression
            ))
        }
        // todo!("Rewrite this method to improve type checking and parse let")
    }

    fn parse_return(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let current_token = Rc::clone(&self.current_token);
        let ret = Return::new(None, current_token);
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

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let current_token = Rc::clone(&self.current_token);
        let mut stmt = ExpressionStatement::new(Type::Unknown, Rc::clone(&current_token));
        stmt.expression = if let Ok(expr) = self.parse_expression(Precedence::Lowest) {
            Some(expr)
        } else {
            None
        };
        if self.peek_token_is(&Token::Semicolon(None)) {
            self.next_token();
        }

        Ok(Box::new(stmt))
    }

    //TODO: improve parse expression to really parse expression
    fn parse_expression(
        &mut self,
        _precedence: Precedence
    ) -> Result<Box<dyn Expression>, ParseError> {
        let token = self.current_token.as_ref();
        let prefix = self.prefix_parse_fns.get(token);
        
        match prefix {
            Some(prefix_fn) => Ok(prefix_fn(self)?),
            None => {
                let msg = format!("invalid expression in {}", self.current_token);
                Err(ParseError::Message(msg))
            }
        }
    }

    fn parse_identifier(parser: &mut Self) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = Rc::clone(&parser.current_token);
        let identifier_value = match current_token.as_ref() {
            Token::Ident(Some(ident), _) => Rc::clone(ident),
            tok => {
                let msg = format!("expected indentifer, got {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        Ok(Box::new(Identifier::new(identifier_value, current_token)))
    }

    fn parse_number_literal(parser: &mut Self) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = Rc::clone(&parser.current_token);
        let number = if let Token::Number(Some(val), _) = parser.current_token.as_ref() {
            val.trim().parse().unwrap()
        } else {
            let msg = format!("fail on parse value: {}", current_token);
            return Err(ParseError::Message(msg));
        };
        let int_expr = IntExpr::new(number, current_token);
        Ok(Box::new(int_expr))
    }

    fn parse_bool_literal(parser: &mut Self) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = Rc::clone(&parser.current_token);
        let boolean = match parser.current_token.as_ref() {
            Token::True(_) => true,
            Token::False(_) => false,
            tok => {
                let msg = format!("expected boolean value got: {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        let bool_expr = BoolExpr::new(boolean, current_token);
        Ok(Box::new(bool_expr))
    }

    fn parse_prefix_expression(parser: &mut Self) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = Rc::clone(&parser.current_token);
        let mut prefix_expr = PrefixExpr::new(Rc::clone(&current_token), current_token.literal());

        parser.next_token();

        prefix_expr.right = if let Ok(expr) = parser.parse_expression(Precedence::Prefix) {
            Some(expr)
        } else {
            None
        };

        Ok(Box::new(prefix_expr))
    }

    fn parse_string_literal(parser: &mut Self) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = Rc::clone(&parser.current_token);
        let string = match parser.current_token.as_ref() {
            Token::String(val, _) => val.as_ref().unwrap().as_ref().trim(),
            tok => {
                let msg = format!("expected boolean value got: {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        let string_expr = StrExpr::new(string.to_string(), current_token);
        Ok(Box::new(string_expr))
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

    fn peek_token_is(&mut self, token: &Token) -> bool {
        *self.peek_token == *token
    }

    fn _current_token_is(&mut self, token: Token) -> bool {
        *self.current_token == token
    }

    fn has_type(&mut self) -> bool {
        self.peek_token_is(&Token::Int(None))
        || self.peek_token_is(&Token::Str(None))
        || self.peek_token_is(&Token::Bool(None))
    }

}
