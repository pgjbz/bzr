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
        let token = if let Some(ch) = &self.ch {
            match ch {
                '=' => {
                    let mut token: Token = Token::Assign(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        token = Token::Eq(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                    } else if !(Self::is_whitespace(Some(next_char)) 
                        && (next_char == '\"' || next_char == '(')
                        && Self::is_number(Some(next_char))
                        && Self::is_letter(Some(next_char)))
                    {
                        token = Token::Illegal(Some(Rc::new(format!("{}{}", ch, next_char))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    }
                    self.read_char();
                    token
                }
                '+' => Token::Plus(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '-' => Token::Minus(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '!' => {
                    let mut token: Token = Token::Bang(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        token = Token::Diff(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                        self.read_char();
                    }
                    token
                }
                '/' => Token::Slash(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '*' => Token::Asterisk(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '<' => {
                    let mut token = Token::Lt(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        self.read_char();
                        token = Token::Lte(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                    }
                    token
                }
                '>' => {
                    let mut token = Token::Gt(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '=' {
                        self.read_char();
                        token = Token::Gte(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    }
                    token
                }
                '&' => {
                    let mut token = Token::Illegal(Some(Rc::new(String::from(self.ch.unwrap()))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '&' {
                        self.read_char();
                        token = Token::And(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    }
                    token
                }
                ';' => Token::Semicolon(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '(' => Token::Lparen(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                ')' => Token::Rparen(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '[' => Token::LSqBracket(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                ']' => Token::RSqBracket(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                ',' => Token::Comma(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '{' => Token::Lbrace(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '}' => Token::Rbrace(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                '\"' => {
                    let string = Self::read_string(self);
                    let value = Some(Rc::new(String::from(string)));
                    match string.chars().last() {
                        Some(ch) => {
                            if ch != '\"' {
                                Token::Illegal(value, Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                            } else {
                                Token::String(Some(Rc::new(String::from(&string[0..string.len() - 1]))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                            }
                        }
                        None => Token::Illegal(value, Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename)))),
                    }
                },
                '|' => {
                    let mut token = Token::Illegal(Some(Rc::new(String::from(self.ch.unwrap()))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    let next_char = Self::peek_next_char(self, None);
                    if next_char == '|' {
                        self.read_char();
                        token = Token::Or(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))));
                    }
                    token
                }
                _ => {
                    let line_position = self.line_position;
                    let line = self.line;
                    let filename = Rc::clone(&self.filename);
                    if Self::is_letter(Some(*ch)) {
                        let ident: &str = Self::read_identifier(self);
                        match Token::get_keyword_token(ident, Some(Location::new(line_position, line, Rc::clone(&filename)))) {
                            Ok(keyword_token) => keyword_token,
                            Err(_) => Token::Ident(Some(Rc::new(String::from(ident))), Some(Location::new(line_position, line, Rc::clone(&filename)))),
                        }
                    } else if Self::is_number(Some(*ch)) {
                        //TODO: improve this
                        let next_char = Self::peek_next_char(self, Some(1));
                        let ident: &str = Self::read_number(self);
                        let mut token: Token = Token::Illegal(Some(Rc::new(String::from(ident))), Some(Location::new(line_position, line, filename)));
                        if Self::is_math_simbol(next_char)
                            || Self::is_whitespace(Some(next_char))
                            || next_char == ';'
                            || next_char == '{'
                            || next_char == '&'
                            || next_char == '\0' //TODO: check if this is necessary
                            || next_char == '!'
                            || next_char == ','
                            || next_char == ']'
                            || Self::is_number(Some(next_char))
                        {
                            token = Token::Number(Some(Rc::new(String::from(ident))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                        } else {
                            self.read_char();
                        }
                        token
                    } else {
                        Token::Illegal(Some(Rc::new(String::from(self.ch.unwrap()))), Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
                    }
                }
            }
        } else {
            Token::EOF(Some(Location::new(self.line_position, self.line, Rc::clone(&self.filename))))
        };
        Rc::new(token)
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
        let ret = &self.input[position..final_pos];
        ret
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
        let ret = &self.input[position..final_pos];
        ret
    }

    fn back_position(&mut self) {
        self.line_position -= 1;
        self.position -= 1;
        self.read_position -= 1;
    }

    fn read_string(input: &mut Self) -> &str {
        let position = input.position + 1;
        input.read_char();
        while input.position < input.input.len() && input.ch != Some('\"') || input.ch == None {
            input.read_char();
        }
        &input.input[position..input.position + 1]
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

    fn is_math_simbol(ch: char) -> bool {
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
