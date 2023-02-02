use super::token::{Token, TokenType, LookupIdentifierType};

pub struct lexer {
	input:  String,
	position: usize,
	next_position: usize,
	ch: char
}

impl lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            position: 0,
            next_position: 0,
            ch: '`',
        }
    }

    fn readChar(&mut self) {
        self.ch = self.peekChar().unwrap();
        self.position = self.next_position;
        self.next_position += 1;
    }
    
    fn readString(&mut self) -> &str {
        let pos = self.position + 1;
        loop {
            self.readChar();
            if self.ch == '"' {
                break;
            }
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn readNumber(&mut self) -> &str {
        let pos = self.position;
        while Self::isDigit(self.ch) {
            self.readChar();
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn readIdentifier(&mut self) -> &str {
        let pos = self.position;
        while Self::isLetter(self.ch) {
            self.readChar();
        }
        return self.input.get(pos..self.position).unwrap();
    }
    
    fn isLetter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }
    
    fn isDigit(ch: char) -> bool {
        return '0' <= ch && ch <= '9'
    }
    
    fn skipWhitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.readChar()
        }
    }
    
    fn peekChar(&self) -> Option<char> {
        match self.input.len().cmp(&self.next_position) {
            std::cmp::Ordering::Less => { None },
            std::cmp::Ordering::Equal => { None},
            std::cmp::Ordering::Greater => {
                 Some(self.input.chars().nth(self.next_position).unwrap())
            }
        }
    }

    fn newToken(tokenType: TokenType, ch: char) -> Token {
        Token{
            Type: tokenType, 
            Literal: ch.to_string()
        }
    }

    
    fn NextToken(&mut self) -> Token {
        let mut tok :Token = Token { Type: TokenType::Bang, Literal: "!".to_string() };

        self.skipWhitespace();

        match self.ch {
            '=' => {
                if self.peekChar().unwrap() == '=' {
                    let ch = self.ch;
                    self.readChar();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token{
                        Type: TokenType::Equal,
                        Literal: literal
                    }
                } else {
                    tok = Self::newToken(TokenType::Assign, self.ch)
                }
            },
            '+' => { tok = Self::newToken(TokenType::Plus, self.ch); },
            '-' => { tok = Self::newToken(TokenType::Minus, self.ch); },
            '!' => { 
                if self.peekChar().unwrap() == '=' {
                    let ch = self.ch;
                    self.readChar();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token{
                        Type: TokenType::NotEqual, 
                        Literal: literal
                    };
                } else {
                    tok = Self::newToken(TokenType::Bang, self.ch);
                }
            },
            '*' => { tok = Self::newToken(TokenType::Asterisk, self.ch); },
            '/' => { tok = Self::newToken(TokenType::Slash, self.ch); },
            '<' => { tok = Self::newToken(TokenType::LessThan, self.ch); },
            '>' => { tok = Self::newToken(TokenType::GreaterThan, self.ch); },
            ',' => { tok = Self::newToken(TokenType::Comma, self.ch); },
            ';' => { tok = Self::newToken(TokenType::Semicolon, self.ch); },
            ':' => { tok = Self::newToken(TokenType::Colon, self.ch); },
            '(' => { tok = Self::newToken(TokenType::LeftParen, self.ch); },
            ')' => { tok = Self::newToken(TokenType::RightParen, self.ch); },
            '{' => { tok = Self::newToken(TokenType::LeftBrace, self.ch); },
            '}' => { tok = Self::newToken(TokenType::RightBrace, self.ch); },
            '[' => { tok = Self::newToken(TokenType::LeftBracket, self.ch); },
            ']' => { tok = Self::newToken(TokenType::RightBracket, self.ch); },
            '"' => {
                tok.Type = TokenType::String;
                tok.Literal = self.readString().to_owned();
            },
            _ => {
                if Self::isLetter(self.ch) {
                    tok.Literal = self.readIdentifier().to_owned();
                    tok.Type = LookupIdentifierType(&tok.Literal);
                    return tok;
                } else if Self::isDigit(self.ch) {
                    tok.Literal = self.readNumber().to_owned();
                    tok.Type = TokenType::Int;
                    return tok;
                } else {
                    tok = Self::newToken(TokenType::Illegal, self.ch);
                }
            }

        }

        self.readChar();
        return tok;
    }
    
}

pub fn New(input: &str) -> lexer {
	let mut l: lexer = lexer::new(input);
	l.readChar();
	return l
}

