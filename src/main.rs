mod ast;
mod lexer;
mod parser;
mod token;

use lexer::*;
use parser::Parser;
use token::Token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cry_1 [filename] [option]\nHelp: cry_1 help");
        std::process::exit(1);
    }

    let filename = &args[1];
    if filename == "--help" || filename == "-h" || filename == "help" {
        help();
        std::process::exit(0);
    }
    let contents = match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Something went wrong reading file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    let mut option = &String::new();
    if args.len() > 2 {
        option = &args[2];
    }
    if option == "--help" || filename == "-h" || filename == "help" {
        help();
        std::process::exit(0);
    }

    let mut lexer = Lexer::new(contents);
    let tokens: Vec<Token> = lexer.lex();

    if option == "--tokens" || option == "-t" || option == "tokens" {
        tokens.iter().for_each(|t| println!("{:?}", t));
        std::process::exit(0);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if option == "--ast" || option == "-a" || option == "ast" {
        println!("{:#?}", ast);
        std::process::exit(0);
    }
}

fn help() {
    println!("=== Cry_1 Assembler made by Nicholas Stienz ===");
    println!("Usage: cry_1 [filename] [option]\n");
    println!("Options:");
    println!("Help:\t\t--help, -h, help\t\t- Prints this help message.");
    println!("Tokens:\t\t--tokens, -t, tokens\t\t- Prints all the tokens found in the program.");
}
