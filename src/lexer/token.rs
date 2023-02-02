
#[derive(Debug)]
pub enum TokenType {
    Illegal,
	EOF,

	// Identifiers + Literals
	Identifier,
	Int,
	String,

	// Operators
	Assign,   
	Plus,  
	Minus,    
	Bang,     
	Asterisk, 
	Slash,    
	Equal,    
	NotEqual, 

	LessThan,
	GreaterThan,

	// Delimiters
	Comma,
	Semicolon,
	Colon,

	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,

	// Keywords
	Function,
	Let,
	True,
	False,
	If,
	Else,
	Return
}

#[derive(Debug)]
pub struct Token {
    pub Type: TokenType,
    pub literal: String
}

impl Token {
    fn new(Type: TokenType, literal: String) -> Token {
        Self {
            Type,
            literal,
        }
    }
}


pub fn lookup_identifier_type(identifier: &str) -> TokenType {
    match identifier {
        "fn" => return TokenType::Function,
        "let" => return TokenType::Let,
        "true" => return TokenType::True,
        "false" => return TokenType::False,
        "if" => return TokenType::If,
        "else" => return TokenType::Else,
        "return" => return TokenType::Return,
        _ => return TokenType::Identifier
    };
}