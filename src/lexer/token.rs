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

pub struct Token {
    pub Type: TokenType,
    pub Literal: String
}

impl Token {
    fn new(Type: TokenType, Literal: String) -> Token {
        Self {
            Type,
            Literal,
        }
    }
}


pub fn LookupIdentifierType(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => unreachable!()
    };
	TokenType::Identifier
}