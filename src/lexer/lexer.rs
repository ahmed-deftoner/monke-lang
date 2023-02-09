use super::token::{Token, TokenType, lookup_identifier_type};

pub struct Lexer {
	input:  String,
	position: usize,
	next_position: usize,
	ch: char
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            position: 0,
            next_position: 0,
            ch: input.chars().nth(0).unwrap(),
        }
    }

    fn read_char(&mut self) {
        if self.peek_char() != None {
            self.ch = self.peek_char().unwrap();
            self.position = self.next_position;
            self.next_position += 1;
        }
    }
    
    fn read_string(&mut self) -> &str {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' {
                break;
            }
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn read_number(&mut self) -> &str {
        let pos = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn read_identifier(&mut self) -> &str {
        let pos = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }
    
    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9'
    }
    
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
    
    fn peek_char(&self) -> Option<char> {
        match self.input.len().cmp(&self.next_position) {
            std::cmp::Ordering::Less => { None },
            std::cmp::Ordering::Equal => { None},
            std::cmp::Ordering::Greater => {
                 Some(self.input.chars().nth(self.next_position).unwrap())
            }
        }
    }

    fn new_token(token_type: TokenType, ch: char) -> Token {
        Token{
            Type: token_type, 
            literal: ch.to_string()
        }
    }

    
    pub fn next_token(&mut self) -> Token {
        let mut tok :Token = Token { Type: TokenType::Bang, literal: "!".to_string() };

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char().unwrap() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token{
                        Type: TokenType::Equal,
                        literal
                    }
                } else {
                    tok = Self::new_token(TokenType::Assign, self.ch)
                }
            },
            '+' => { tok = Self::new_token(TokenType::Plus, self.ch); },
            '-' => { tok = Self::new_token(TokenType::Minus, self.ch); },
            '!' => { 
                if self.peek_char().unwrap() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token{
                        Type: TokenType::NotEqual, 
                        literal
                    };
                } else {
                    tok = Self::new_token(TokenType::Bang, self.ch);
                }
            },
            '*' => { tok = Self::new_token(TokenType::Asterisk, self.ch); },
            '/' => { tok = Self::new_token(TokenType::Slash, self.ch); },
            '<' => { tok = Self::new_token(TokenType::LessThan, self.ch); },
            '>' => { tok = Self::new_token(TokenType::GreaterThan, self.ch); },
            ',' => { tok = Self::new_token(TokenType::Comma, self.ch); },
            ';' => { tok = Self::new_token(TokenType::Semicolon, self.ch); },
            ':' => { tok = Self::new_token(TokenType::Colon, self.ch); },
            '(' => { tok = Self::new_token(TokenType::LeftParen, self.ch); },
            ')' => { tok = Self::new_token(TokenType::RightParen, self.ch); },
            '{' => { tok = Self::new_token(TokenType::LeftBrace, self.ch); },
            '}' => { tok = Self::new_token(TokenType::RightBrace, self.ch); },
            '[' => { tok = Self::new_token(TokenType::LeftBracket, self.ch); },
            ']' => { tok = Self::new_token(TokenType::RightBracket, self.ch); },
            '"' => {
                tok.Type = TokenType::String;
                tok.literal = self.read_string().to_owned();
            },
            _ => {
                if Self::is_letter(self.ch) {
                    tok.literal = self.read_identifier().to_owned();
                    tok.Type = lookup_identifier_type(&tok.literal);
                    return tok;
                } else if Self::is_digit(self.ch) {
                    tok.literal = self.read_number().to_owned();
                    tok.Type = TokenType::Int;
                    return tok;
                } else {
                    tok = Self::new_token(TokenType::EOF, '/');
                }
            }

        }

        self.read_char();
        return tok;
    }
    
}

pub fn new_lex(input: &str) -> Lexer {
	let mut l: Lexer = Lexer::new(input);
	l.read_char();
	return l
}

