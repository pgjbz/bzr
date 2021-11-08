use self::token::{Location, Token};

pub mod token;

pub struct Lexer<'a> {
    input: &'a str, //Source code
    position: usize,
    read_position: usize,
    ch: Option<char>,
    line: usize,
    line_position: usize,
    filename: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, filename: &'a str) -> Self {
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

    pub fn next_token(&mut self) -> Box<Token<'a>> {
        self.read_char();
        self.skip_whitespace();
        let location = Location::new(self.line_position, self.line, self.filename);
        let token = if let Some(ch) = &self.ch {
            match ch {
                '=' => {
                    let mut token: Token = Token::Assign(location);
                    if let Some(nch) = self.input.chars().nth(self.read_position) {
                        if nch == '=' {
                            self.read_char();
                            token = Token::Eq(location)
                        } else if nch == '>' || nch == '<' {
                            token = Token::Illegal(format!("{}{}", ch, nch), location);
                            self.read_char();
                        }
                    }
                    token
                }
                '+' => Token::Plus(location),
                '-' => Token::Minus(location),
                '!' => Token::Bang(location),
                '/' => Token::Slash(location),
                '*' => Token::Asterisk(location),
                '<' => {
                    let mut token = Token::Lt(location);
                    if let Some(ch) = self.input.chars().nth(self.read_position) {
                        if ch == '=' {
                            self.read_char();
                            token = Token::Lte(location)
                        }
                    }
                    token
                }
                '>' => {
                    let mut token = Token::Gt(location);
                    if let Some(ch) = self.input.chars().nth(self.read_position) {
                        if ch == '=' {
                            self.read_char();
                            token = Token::Gte(location);
                        }
                    }
                    token
                }
                ';' => Token::Semicolon(location),
                '(' => Token::Lparen(location),
                ')' => Token::Rparen(location),
                ',' => Token::Comma(location),
                '{' => Token::Lbrace(location),
                '}' => Token::Rbrace(location),
                '\"' => {
                    let string = Self::read_string(self);
                    let value = String::from(string);
                    match string.chars().last() {
                        Some(ch) => {
                            if ch != '\"' {
                                Token::Illegal(value, location)
                            } else {
                                Token::String(String::from(&string[0..string.len() - 1]), location)
                            }
                        }
                        None => Token::Illegal(value, location),
                    }
                }
                _ => {
                    let read_position = self.read_position;
                    let content = self.input;
                    if is_letter(Some(*ch)) {
                        let ident: &str = Self::read_identifier(self);
                        match Token::get_keyword_token(ident, location) {
                            Ok(keyword_token) => keyword_token,
                            Err(_) => Token::Ident(String::from(ident), location),
                        }
                    } else if is_number(Some(*ch)) {
                        let ident: &str = Self::read_number(self);
                        let mut token: Token = Token::Illegal(String::from(ident), location);
                        if let Some(ch) = content.chars().nth(read_position + 1) {
                            if is_math_simbol(ch)
                                || is_whitespace(Some(ch))
                                || ch == ';'
                                || ch == '{'
                            {
                                token = Token::Number(String::from(ident), location)
                            } else {
                                self.read_char();
                            }
                        }
                        token
                    } else {
                        Token::Illegal(String::from(self.ch.unwrap()), location)
                    }
                }
            }
        } else {
            Token::EOF(location)
        };
        Box::new(token)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if is_whitespace(self.ch) {
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
            } else {
                self.line_position += 1;
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(input: &mut Self) -> &str {
        let position = input.position;
        while input.position < input.input.len() && is_number(input.ch) {
            input.read_char();
        }
        let ret = &input.input[position..input.position];
        Self::back_position(input);
        ret
    }

    fn read_identifier(input: &mut Self) -> &str {
        let position = input.position;
        while input.position < input.input.len() && is_letter(input.ch) {
            input.read_char();
        }
        let ret = &input.input[position..input.position];
        Self::back_position(input);
        ret
    }

    fn back_position(input: &mut Self) {
        input.line_position -= 1;
        input.position -= 1;
        input.read_position -= 1;
    }

    fn read_string(input: &mut Self) -> &str {
        let position = input.position + 1;
        input.read_char();
        while input.position < input.input.len() && input.ch != Some('\"') || input.ch == None {
            input.read_char();
        }
        &input.input[position..input.position + 1]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Box<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        if let token::Token::EOF(_) = *next {
            None
        } else {
            Some(next)
        }
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
        ('0'..='0').contains(&ch)
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
