mod errors;
mod precedence;

use std::{collections::HashMap, mem, rc::Rc};

use crate::{
    ast::{
        expr::{
            arr_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr,
            function_expr::FunctionExpr, if_expr::IfExpr, index_expr::IndexExpr,
            infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr, str_expr::StrExpr,
            while_expr::WhileExpr,
        },
        expression::Expression,
        identifier::Identifier,
        program::Program,
        statement::Statement,
        stmt::{
            block_stmt::BlockStatement, expression_stmt::ExpressionStatement, let_stmt::Let,
            return_stmt::Return, var_stmt::Var,
        },
        types::Type,
    },
    lexer::{token::Token, Lexer},
};

use self::{errors::ParseError, precedence::Precedence};

type PrefixParseFn = fn(&mut Parser) -> Result<Rc<dyn Expression>, ParseError>;
type InfixParseFn = fn(&mut Parser, Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError>;

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
        prefix_parse_fns.insert(Token::LParen(None), Self::parse_grouped_expression);
        prefix_parse_fns.insert(Token::LSqBracket(None), Self::parse_array);
        prefix_parse_fns.insert(Token::If(None), Self::parse_if_expression);
        prefix_parse_fns.insert(Token::While(None), Self::parse_while_expression);
        prefix_parse_fns.insert(Token::Function(None), Self::parse_function_literal);

        infix_parse_fns.insert(Token::Plus(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Minus(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Slash(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::ShiftLeft(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::ShiftRight(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::BitWiseAnd(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::BitWiseOr(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Xor(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Asterisk(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Mod(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Eq(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Diff(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Lt(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Or(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Gt(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Gte(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Lte(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::And(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::Assign(None), Self::parse_infix_expression);
        infix_parse_fns.insert(Token::LParen(None), Self::parse_call_expression);
        infix_parse_fns.insert(Token::LSqBracket(None), Self::parse_index_expression);
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
        loop {
            match self.parse_statement() {
                Ok(sts) => {
                    self.next_token();
                    statements.push(sts)
                }
                Err(e) => match e {
                    ParseError::Eof => break,
                    ParseError::Message(msg) => {
                        self.next_token();
                        self.errors.push(msg)
                    }
                },
            }
        }
        Program::new(statements, self.errors)
    }

    fn parse_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
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

    fn parse_let_var(&mut self, is_let: bool) -> Result<Rc<dyn Statement>, ParseError> {
        let current_token = Rc::clone(&self.current_token);
        self.expected_peek(Token::Ident(None, None))?;
        let identifier: Rc<dyn Expression> = Self::parse_identifier(self)?;
        let expression: Rc<dyn Expression>;
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
        if self.peek_token_is(&Token::Semicolon(None)) {
            self.next_token();
        }
        if is_let {
            Ok(Let::new(expression.get_type(), identifier, expression))
        } else {
            Ok(Var::new(expression.get_type(), identifier, expression))
        }
    }

    fn parse_return(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        let mut ret = Return::new(None);
        self.next_token();
        ret.return_value = Some(self.parse_expression(Precedence::Lowest)?);
        self.next_token();
        Ok(Rc::new(ret))
    }

    fn parse_expression_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        let mut stmt = ExpressionStatement::new(Type::Unknown);
        stmt.expression = match self.parse_expression(Precedence::Lowest) {
            Ok(expr) => Some(expr),
            Err(e) => match e {
                ParseError::Message(_) => return Err(e),
                _ => None,
            },
        };
        if self.peek_token_is(&Token::Semicolon(None)) {
            self.next_token();
        }

        Ok(Rc::new(stmt))
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Rc<dyn Expression>, ParseError> {
        let token = self.current_token.as_ref();
        let prefix = self.prefix_parse_fns.get(token);
        let mut left_expr = match prefix {
            Some(prefix_fn) => prefix_fn(self)?,
            None => {
                let msg = format!("error expected a prefix, got {}", self.current_token);
                return Err(ParseError::Message(msg));
            }
        };

        while !self.current_token_is(Token::Semicolon(None))
            && precedence < precedence::get_precedence(self.peek_token.as_ref())
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

    fn parse_identifier(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let identifier_expr = parser.create_identifier(false)?;
        Ok(Rc::clone(&identifier_expr))
    }

    fn create_identifier(&mut self, skip_type: bool) -> Result<Rc<dyn Expression>, ParseError> {
        let identifier_value = match self.current_token.as_ref() {
            Token::Ident(Some(ident), _) => Rc::clone(ident),
            tok => {
                let msg = format!("expected identifier, got {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        let mut identifier_expr = Identifier::new(identifier_value);
        if self.has_type() {
            let typ = self.peek_token.to_type();
            if skip_type {
                self.next_token();
            }
            identifier_expr.set_type(typ);
        }
        Ok(Rc::new(identifier_expr))
    }

    fn parse_number_literal(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let number = match parser.current_token.as_ref() {
            Token::Number(Some(val), _) => val.trim().parse()?,
            _ => {
                let msg = format!("fail on parse value: {}", parser.current_token);
                return Err(ParseError::Message(msg));
            }
        };
        let int_expr = IntExpr::new(number);
        Ok(Rc::new(int_expr))
    }

    fn parse_bool_literal(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let boolean = match parser.current_token.as_ref() {
            Token::True(_) => true,
            Token::False(_) => false,
            tok => {
                let msg = format!("expected boolean value got: {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        let bool_expr = BoolExpr::new(boolean);
        Ok(Rc::new(bool_expr))
    }

    fn parse_function_literal(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();
        let identifier = Self::parse_identifier(parser)?;
        let mut function_expr = FunctionExpr::new(identifier);
        parser.expected_peek(Token::LParen(None))?;
        function_expr.parameters = parser.parse_function_parameters()?;
        if parser.has_type() {
            parser.next_token();
            function_expr.ret_typ = match parser.current_token.as_ref() {
                Token::Bool(_) => Type::Bool,
                Token::Int(_) => Type::Int,
                Token::Str(_) => Type::String,
                _ => Type::Unknown,
            };
        }
        parser.expected_peek(Token::LBrace(None))?;
        function_expr.body = parser.parse_block_statement();
        Ok(Rc::new(function_expr))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Rc<dyn Expression>>, ParseError> {
        let mut identifiers = Vec::<Rc<dyn Expression>>::with_capacity(5);
        if self.peek_token_is(&Token::RParen(None)) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();
        let identifier = self.create_identifier(true)?;
        identifiers.push(identifier);

        while self.peek_token_is(&Token::Comma(None)) {
            self.next_token();
            self.next_token();
            let identifier = self.create_identifier(true)?;
            identifiers.push(identifier);
        }
        // self.next_token();
        self.expected_peek(Token::RParen(None))?;

        Ok(identifiers)
    }

    fn parse_prefix_expression(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut prefix_expr = PrefixExpr::new(parser.current_token.literal());
        prefix_expr.set_type(parser.current_token.to_type());
        parser.next_token();

        prefix_expr.right = match parser.parse_expression(Precedence::Prefix) {
            Ok(expr) => Some(expr),
            Err(e) => match e {
                ParseError::Message(_) => return Err(e),
                _ => None,
            },
        };

        Ok(Rc::new(prefix_expr))
    }

    fn parse_string_literal(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let string = match parser.current_token.as_ref() {
            Token::String(val, _) => val.as_ref().unwrap().as_ref(),
            tok => {
                let msg = format!("expected boolean value got: {}", tok);
                return Err(ParseError::Message(msg));
            }
        };
        let string_expr = StrExpr::new(string.to_string());
        Ok(Rc::new(string_expr))
    }

    fn parse_if_expression(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();

        let expr = parser.parse_expression(Precedence::Lowest)?;

        parser.expected_peek(Token::LBrace(None))?;

        let consequence_block = parser.parse_block_statement();

        let mut if_expr = IfExpr::new(expr);
        if_expr.consequence = consequence_block;

        if parser.peek_token_is(&Token::Else(None)) {
            parser.next_token();
            match parser.expected_peek(Token::LBrace(None)) {
                Ok(_) => {
                    let alternative_block = parser.parse_block_statement();
                    if_expr.alternative = alternative_block;
                }
                Err(_) => match parser.expected_peek(Token::If(None)) {
                    Ok(_) => if_expr.el_if = Some(Self::parse_if_expression(parser)?),
                    Err(e) => return Err(e),
                },
            }
        }

        Ok(Rc::new(if_expr))
    }

    fn parse_while_expression(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();
        let expr = parser.parse_expression(Precedence::Lowest)?;
        parser.expected_peek(Token::LBrace(None))?;
        let consequence_block = parser.parse_block_statement();
        let mut while_expr = WhileExpr::new(expr);
        while_expr.consequence = consequence_block;
        Ok(Rc::new(while_expr))
    }

    fn parse_block_statement(&mut self) -> Option<Rc<BlockStatement>> {
        let mut block_stmt = BlockStatement::new();

        while !self.current_token_is(Token::Rbrace(None))
            && !self.current_token_is(Token::EOF(None))
        {
            let statement = self.parse_statement();
            if let Ok(statement) = statement {
                block_stmt.push_stmt(statement);
            }
            self.next_token();
        }

        Some(Rc::new(block_stmt))
    }

    fn parse_infix_expression(
        parser: &mut Self,
        left: Rc<dyn Expression>,
    ) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();
        let precedence = precedence::get_precedence(parser.current_token.as_ref());
        let mut infix_expr = InfixExpr::new(parser.current_token.literal());
        infix_expr.left = Some(left);
        let typ = parser.current_token.to_type();
        infix_expr.set_type(typ);
        parser.next_token();
        infix_expr.right = match parser.parse_expression(precedence) {
            Ok(expr) => Some(expr),
            Err(e) => return Err(e),
        };

        Ok(Rc::new(infix_expr))
    }

    fn parse_index_expression(
        parser: &mut Self,
        left: Rc<dyn Expression>,
    ) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();
        parser.next_token();
        let idx_expr = parser.parse_expression(Precedence::Lowest)?;
        parser.expected_peek(Token::RSqBracket(None))?;
        Ok(Rc::new(IndexExpr::new(left, idx_expr)))
    }

    fn parse_call_expression(
        parser: &mut Self,
        function: Rc<dyn Expression>,
    ) -> Result<Rc<dyn Expression>, ParseError> {
        let mut call_expr = CallExpr::new(function);
        parser.next_token();
        call_expr.arguments = parser.parse_expr_list(Token::RParen(None))?;
        Ok(Rc::new(call_expr))
    }

    fn parse_grouped_expression(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        parser.next_token();

        let expr = parser.parse_expression(Precedence::Lowest);

        match parser.expected_peek(Token::RParen(None)) {
            Ok(_) => {}
            Err(_) => {
                let msg = format!("expected LParen got, {}", parser.peek_token);
                return Err(ParseError::Message(msg));
            }
        }

        expr
    }

    fn parse_array(parser: &mut Self) -> Result<Rc<dyn Expression>, ParseError> {
        let exprs = parser.parse_expr_list(Token::RSqBracket(None))?;
        let array_expr = ArrayExpr::new(exprs);
        Ok(Rc::new(array_expr))
    }

    fn parse_expr_list(&mut self, end: Token) -> Result<Vec<Rc<dyn Expression>>, ParseError> {
        let mut exprs = Vec::new();
        if self.peek_token_is(&end) {
            self.next_token();
            return Ok(exprs);
        }

        self.next_token();
        exprs.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token_is(&Token::Comma(None)) {
            self.next_token();
            self.next_token();
            exprs.push(self.parse_expression(Precedence::Lowest)?);
        }
        self.expected_peek(end)?;
        Ok(exprs)
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
            || self.peek_token_is(&Token::Array(None))
    }
}
