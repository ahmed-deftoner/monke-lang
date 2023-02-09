use std::io;

use crate::lexer::{token::TokenType, lexer::new_lex};


const PROMPT: &str = ">>";

pub fn start() {
    loop {
        println!("{}", PROMPT);
        let mut expr = String::new();
        io::stdin().read_line(&mut expr).expect("failed to readline");  
        print!("{}", expr); 
        let mut lex = new_lex(&expr);
        let mut tok = lex.next_token();
        while tok.Type != TokenType::EOF {
            println!("inside {:?}", tok);
            tok = lex.next_token();
        }  
        println!("after")
    }

}