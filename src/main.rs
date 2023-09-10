mod lexer;
mod token;

use lexer::*;
use crate::token::Token;

fn main() {
    // This will be the assembler for the CRY-1 Computer so buckle up and enjoy your ride to hell!
    println!("=== CRY-1 Assembler v0.2.0 by Nicholas Stienz ===");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cry-1 [filename] [options]");
        std::process::exit(1);
    }
    let filename = &args[1];

    let contents = match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Something went wrong reading file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    let lexer = Lexer::new(contents);
    let tokens: Vec<Token> = lexer.filter_map(|t| Some(t)).collect();

    println!("{:#?}", tokens);
}
