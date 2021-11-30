pub mod errors;
pub mod precedence;

use std::{collections::HashMap, mem, rc::Rc};

use crate::{
    ast::{
        expr::{
            bool_expr::BoolExpr, infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
            str_expr::StrExpr,
        },
        expression::Expression,
        identifier::Identifier,
        program::Program,
        statement::Statement,
        stmt::{
            expression_stmt::ExpressionStatement, let_stmt::Let, return_stmt::Return, var_stmt::Var,
        },
        types::Type
    },
    lexer::{token::Token, Lexer},
};

use self::{errors::ParseError, precedence::Precedence};

type PrefixParseFn = fn(&mut Parser) -> Result<Box<dyn Expression>, ParseError>;
type InfixParseFn =
    fn(&mut Parser, Box<dyn Expression>) -> Result<Box<dyn Expression>, ParseError>;

pub struct Parser {
    lexer: Lexer,
    current_token: Rc<Token>,
    peek_token: Rc<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<Token, PrefixParseFn>,
    infix_parse_fns: HashMap<Token, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut prefix_parse_fns: HashMap<Token, PrefixParseFn> = HashMap::new();
        let mut infix_parse_fns: HashMap<Token, InfixParseFn> = HashMap::new();
        prefix_parse_fns.insert(Token::Ident(None, None), Self::parse_identifier);
        prefix_parse_fns.insert(Token::Number(None, None), Self::parse_number_literal);
        prefix_parse_fns.insert(Token::Minus(None), Self::parse_prefix_expression);
        prefix_parse_fns.insert(Token::Bang(None), Self::parse_prefix_expression);
        prefix_parse_fns.insert(Token::True(None), Self::parse_bool_literal);
        prefix_parse_fns.insert(Token::False(None), Self::parse_bool_literal);
        prefix_parse_fns.insert(Token::String(None, None), Self::parse_string_literal);

        infix_parse_fns.insert(Token::Plus(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Minus(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Slash(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Asterisk(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Eq(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Diff(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Lt(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Gt(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Gte(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Lte(None), Self::parse_infix_expression);
        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parse_fns,
            infix_parse_fns,
        }
    }

    pub fn parse_program(mut self) -> Box<Program> {
        let mut statements = vec![];
        'parse: loop {
            match self.parse_statement() {
                Ok(sts) => {
                    self.next_token();
                    statements.push(sts)
                }
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
            _ => self.parse_expression_statement(),
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
                return Err(ParseError::Message(msg));
            };
            if declared_type.unwrap() != expression.get_type() {
                let msg = format!(
                    "expected {}, got {}",
                    declared_type.unwrap(),
                    expression.get_type()
                );
                return Err(ParseError::Message(msg));
            }
        } else {
            self.expected_peek(Token::Assign(None))?;
            self.next_token();
            expression = match self.parse_expression(Precedence::Lowest) {
                Ok(expr) => expr,
                Err(e) => {
                    let error_msg = if let ParseError::Message(msg) = e {
                        msg
                    } else {
                        "".to_string()
                    };
                    let msg = format!(
                        "error on parse expression {}... {}",
                        current_token, error_msg
                    );
                    return Err(ParseError::Message(msg));
                }
            }
        }
        if is_let {
            Ok(Let::new(
                current_token,
                expression.get_type(),
                identifier,
                expression,
            ))
        } else {
            Ok(Var::new(
                current_token,
                expression.get_type(),
                identifier,
                expression,
            ))
        }
        // todo!("Rewrite this method to improve type checking and parse let")
    }

    fn parse_return(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let current_token = Rc::clone(&self.current_token);
        let ret = Return::new(None, current_token);
        //TODO: we're skipping expression until found a semicolon
        while !self.current_token_is(Token::Semicolon(None)) {
            self.next_token();
            if self.current_token_is(Token::EOF(None)) {
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
        precedence: Precedence,
    ) -> Result<Box<dyn Expression>, ParseError> {
        let token = self.current_token.as_ref();
        let prefix = self.prefix_parse_fns.get(token);
        let mut left_expr = match prefix {
            Some(prefix_fn) => prefix_fn(self)?,
            None => {
                let msg = format!("error expeceted a prefix, got {}", self.current_token);
                return Err(ParseError::Message(msg));
            }
        };

        while !self.current_token_is(Token::Semicolon(None))
            && precedence < precedence::get_precendence(self.peek_token.as_ref())
        {
            let infix = if let Some(infix) = self.infix_parse_fns.get(self.peek_token.as_ref()) {
                infix
            } else {
                return Ok(left_expr);
            };
            left_expr = infix(self, left_expr)?;
        }
        Ok(left_expr)
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

        prefix_expr.right = match parser.parse_expression(Precedence::Prefix) {
            Ok(expr) => Some(expr),
            Err(_) => None,
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

    //TODO: add type checking
    fn parse_infix_expression(
        parser: &mut Self,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, ParseError> {
        parser.next_token();
        let current_token = Rc::clone(&parser.current_token);
        let precedence = precedence::get_precendence(parser.current_token.as_ref());
        let mut infix_expr = InfixExpr::new(current_token, parser.current_token.literal());
        infix_expr.left = Some(left);
        parser.next_token();
        infix_expr.right = match parser.parse_expression(precedence) {
            Ok(expr) => Some(expr),
            Err(e) => return Err(e),
        };

        Ok(Box::new(infix_expr))
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

    fn current_token_is(&mut self, token: Token) -> bool {
        *self.current_token == token
    }

    fn has_type(&mut self) -> bool {
        self.peek_token_is(&Token::Int(None))
            || self.peek_token_is(&Token::Str(None))
            || self.peek_token_is(&Token::Bool(None))
    }
}
