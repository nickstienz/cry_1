mod lexer;
mod token;
use lexer::*;

use crate::token::TokenType;

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
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Error: File {} not found", filename);
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("Error: Permission denied for file {}", filename);
                std::process::exit(1);
            }
            _ => {
                println!("Something went wrong reading file {}: {}", filename, error);
                std::process::exit(1);
            }
        },
    };

    let mut lexer = Lexer::new(contents);

    loop {
        let token = lexer.next();
        match token.token_type {
            TokenType::EOF => {
                println!("{:?}", token);
                break;
            },
            _ => {
                println!("{:?}", token);
            }
        }
    }
}
