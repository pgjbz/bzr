use std::rc::Rc;

use self::token::{Location, Token};

pub mod token;

#[derive(Debug)]
pub struct Lexer {
    input: Rc<String>, //Source code
    position: usize,
    read_position: usize,
    ch: Option<char>,
    line: usize,
    line_position: usize,
    filename: Rc<String>,
}

impl Lexer {
    pub fn new(input: Rc<String>, filename: Rc<String>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            line: 1,
            line_position: 0,
            filename,
        }
    }

    pub fn next_token(&mut self) -> Rc<Token> {
        self.read_char();
        self.skip_whitespace();
        let line_position = self.line_position - 1;
        let line = self.line;
        let filename = Rc::clone(&self.filename);
        let token = if let Some(ch) = &self.ch {
            match ch {
                '=' => {
                    let mut token: Token = Token::Assign(Some(Location::new(
                        line_position,
                        line,
                        Rc::clone(&filename),
                    )));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        token = Token::Eq(Some(Location::new(
                            line_position,
                            line,
                            Rc::clone(&filename),
                        )))
                    } else if !(Self::is_whitespace(Some(next_char))
                        || (next_char == '\"' || next_char == '(')
                        || Self::is_number(Some(next_char))
                        || Self::is_letter(Some(next_char)))
                    {
                        token = Token::Illegal(
                            Some(Rc::new(format!("{}{}", ch, next_char))),
                            Some(Location::new(line_position, line, filename)),
                        );
                    }
                    self.read_char();
                    token
                }
                '+' => Token::Plus(Some(Location::new(line_position, line, filename))),
                '-' => {
                    let mut token = Token::Minus(Some(Location::new(
                        line_position,
                        line,
                        Rc::clone(&filename),
                    )));
                    let next_char = Self::peek_next_char(self, None);
                    if Self::is_number(Some(next_char)) {
                        self.read_char();
                        let ident: &str = self.read_number();
                        if Self::valid_number_suffix(Some(next_char)) {
                            token = Token::Number(
                                Some(Rc::new(format!("-{}", ident))),
                                Some(Location::new(line_position, line, filename)),
                            )
                        } else {
                            self.read_char();
                        }
                    }
                    token
                }
                '!' => {
                    let mut token: Token = Token::Bang(Some(Location::new(
                        line_position,
                        line,
                        Rc::clone(&filename),
                    )));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        token = Token::Diff(Some(Location::new(line_position, line, filename)));
                        self.read_char();
                    }
                    token
                }
                '/' => Token::Slash(Some(Location::new(line_position, line, filename))),
                '*' => Token::Asterisk(Some(Location::new(line_position, line, filename))),
                '<' => {
                    let mut token = Token::Lt(Some(Location::new(
                        line_position,
                        line,
                        Rc::clone(&filename),
                    )));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        self.read_char();
                        token = Token::Lte(Some(Location::new(line_position, line, filename)))
                    }
                    token
                }
                '>' => {
                    let mut token = Token::Gt(Some(Location::new(
                        line_position,
                        line,
                        Rc::clone(&filename),
                    )));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        self.read_char();
                        token = Token::Gte(Some(Location::new(line_position, line, filename)));
                    }
                    token
                }
                '&' => {
                    let mut token = Token::Illegal(
                        Some(Rc::new(String::from(self.ch.unwrap()))),
                        Some(Location::new(line_position, line, Rc::clone(&filename))),
                    );
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '&' {
                        self.read_char();
                        token = Token::And(Some(Location::new(line_position, line, filename)));
                    }
                    token
                }
                ';' => Token::Semicolon(Some(Location::new(line_position, line, filename))),
                '(' => Token::LParen(Some(Location::new(line_position, line, filename))),
                ')' => Token::RParen(Some(Location::new(line_position, line, filename))),
                '[' => Token::LSqBracket(Some(Location::new(line_position, line, filename))),
                ']' => Token::RSqBracket(Some(Location::new(line_position, line, filename))),
                ',' => Token::Comma(Some(Location::new(line_position, line, filename))),
                '{' => Token::LBrace(Some(Location::new(line_position, line, filename))),
                '}' => Token::Rbrace(Some(Location::new(line_position, line, filename))),
                '\"' => {
                    let string = Self::read_string(self);
                    let value = Some(Rc::new(String::from(string)));
                    match string.chars().last() {
                        Some(ch) => {
                            if ch != '\"' {
                                Token::Illegal(
                                    value,
                                    Some(Location::new(line_position, line, filename)),
                                )
                            } else {
                                Token::String(
                                    Some(Rc::new(String::from(&string[0..string.len() - 1]))),
                                    Some(Location::new(line_position, line, filename)),
                                )
                            }
                        }
                        None => Token::Illegal(
                            value,
                            Some(Location::new(line_position, line, filename)),
                        ),
                    }
                }
                '|' => {
                    let mut token = Token::Illegal(
                        Some(Rc::new(String::from(self.ch.unwrap()))),
                        Some(Location::new(line_position, line, Rc::clone(&filename))),
                    );
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '|' {
                        self.read_char();
                        token = Token::Or(Some(Location::new(line_position, line, filename)));
                    }
                    token
                }
                _ => {
                    if Self::is_letter(Some(*ch)) {
                        let ident: &str = Self::read_identifier(self);
                        match Token::get_keyword_token(
                            ident,
                            Some(Location::new(line_position, line, Rc::clone(&filename))),
                        ) {
                            Ok(keyword_token) => keyword_token,
                            Err(_) => Token::Ident(
                                Some(Rc::new(String::from(ident))),
                                Some(Location::new(line_position, line, filename)),
                            ),
                        }
                    } else if Self::is_number(Some(*ch)) {
                        let next_char = Self::peek_next_char(self, Some(1));
                        let ident: &str = self.read_number();
                        let mut token: Token = Token::Illegal(
                            Some(Rc::new(String::from(ident))),
                            Some(Location::new(line_position, line, Rc::clone(&filename))),
                        );
                        if Self::valid_number_suffix(Some(next_char)) {
                            token = Token::Number(
                                Some(Rc::new(String::from(ident))),
                                Some(Location::new(line_position, line, filename)),
                            )
                        } else {
                            self.read_char();
                        }
                        token
                    } else {
                        Token::Illegal(
                            Some(Rc::new(String::from(self.ch.unwrap()))),
                            Some(Location::new(line_position, line, filename)),
                        )
                    }
                }
            }
        } else {
            Token::EOF(Some(Location::new(
                line_position,
                line,
                Rc::clone(&filename),
            )))
        };
        Rc::new(token)
    }

    fn valid_number_suffix(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            Self::is_math_symbol(ch)
                || Self::is_whitespace(Some(ch))
                || ch == ';'
                || ch == '{'
                || ch == '&'
                || ch == '\0'
                || ch == '!'
                || ch == ','
                || ch == ']'
                || Self::is_number(Some(ch))
        } else {
            false
        }
    }
    fn skip_whitespace(&mut self) {
        loop {
            if Self::is_whitespace(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);

        if let Some(ch) = self.ch {
            if ch == '\n' {
                self.line += 1;
                self.line_position = 0;
            }
        }
        self.line_position += 1;
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while self.position < self.input.len() && Self::is_number(self.ch) {
            self.read_char();
        }
        let final_pos = self.position;
        self.back_position();
        &self.input[position..final_pos]
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while self.position < self.input.len() && Self::is_letter(self.ch)
            || Self::is_number(self.ch)
        {
            self.read_char();
        }
        let final_pos = self.position;
        self.back_position();
        &self.input[position..final_pos]
    }

    fn back_position(&mut self) {
        self.line_position -= 1;
        self.position -= 1;
        self.read_position -= 1;
    }

    fn read_string(input: &mut Self) -> &str {
        let position = input.position + 1;
        input.read_char();
        while input.position < input.input.len() && (input.ch != Some('\"') || input.ch == None) {
            input.read_char();
        }
        let mut last_position = input.position + 1;
        while last_position > input.input.len() {
            last_position -= 1;
        }
        &input.input[position..last_position]
    }

    fn peek_next_char(input: &Self, offset: Option<usize>) -> char {
        let offset = if let Some(offset) = offset { offset } else { 0 };
        if let Some(ch) = input.input.chars().nth(input.read_position + offset) {
            ch
        } else {
            '\0'
        }
    }

    fn is_letter(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
        } else {
            false
        }
    }

    fn is_number(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ('0'..='9').contains(&ch)
        } else {
            false
        }
    }

    fn is_whitespace(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
        } else {
            false
        }
    }

    fn is_math_symbol(ch: char) -> bool {
        ch == '*'
            || ch == '/'
            || ch == '+'
            || ch == '-'
            || ch == ')'
            || ch == '('
            || ch == '>'
            || ch == '<'
            || ch == '='
    }
}

impl Iterator for Lexer {
    type Item = Rc<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        if let token::Token::EOF(_) = *next {
            None
        } else {
            Some(next)
        }
    }
}
