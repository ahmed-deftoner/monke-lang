use monke_lang::lexer::lexer::{new_lex};

fn main() {
    println!("Hello, world!");
    let input = include_str!("data.txt");
    let mut lex = new_lex(input);
    let mut x = 0;
    while x < input.len() {
        let tok = lex.next_token();
        println!("{:?}", tok);
        x += 1;
    }
}
