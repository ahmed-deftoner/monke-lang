use std::collections::HashMap;

enum TokenType {
    Illegal,
	EOF,

	// Identifiers + Literals
	Identifier(String),
	Int(i32),
	String(String),

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

struct Token {
    Type: TokenType,
    Literal: String
}

