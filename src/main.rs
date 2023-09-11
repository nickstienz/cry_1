mod lexer;
mod token;

use lexer::*;

fn main() {
    println!("=== CRY-1 Assembler by Nicholas Stienz ===");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cry_1 [filename] [option]");
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

    let mut option = &String::new();
    if args.len() > 2 {
        option = &args[2];
    }

    let mut lexer = Lexer::new(contents);

    if option == "--tokens" || option == "-t" {
        while let Some(token) = lexer.next() {
            println!("{:?}", token);
        }
        return;
    }
}
