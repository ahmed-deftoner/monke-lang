
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
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::String
    };
	TokenType::Identifier
}