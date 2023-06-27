mod instructions;
use instructions::*;

fn main() {
    // This will be the assembler for the CRY-1 Computer so buckle up and enjoy your ride to hell!
    println!("=== CRY-1 Assembler v0.1.0 by Nicholas Stienz ===");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cry-1 <file>");
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Assembling file: {}", filename);

    let contents_result = std::fs::read_to_string(filename);
    let contents = match contents_result {
        Ok(contents) => contents,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Error: File not found: {}", filename);
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("Error: Permission denied: {}", filename);
                std::process::exit(1);
            }
            _ => {
                println!("Error: Can not read file: {}", filename);
                std::process::exit(1);
            }
        },
    };

    let mut program: Vec<String> = Vec::with_capacity(contents.lines().count());
    for line in contents.lines() {
        program.push(line.to_string().trim().to_string());
    }
    program.retain(|line| !line.is_empty() && !line.starts_with(';'));
    for line in program.iter_mut() {
        if let Some(index) = line.find(';') {
            *line = line[..index].to_string().trim().to_string();
        }
    }

    let start_index = program.iter().position(|line| line == "_start:");
    let start_index = match start_index {
        Some(index) => index,
        None => {
            println!("Error: No _start label found");
            std::process::exit(1);
        }
    };
    program.remove(start_index);

    #[cfg(debug_assertions)]
    {
        println!("Index: {}", start_index);
        println!("Program: {:#?}", program);
    }

    let mut new_program: Vec<Instructions> = Vec::with_capacity(program.len());
    program.iter().for_each(|line| {
        let mut tokens = line.split_whitespace();
        let instruction = tokens.next().unwrap();

        let data = if let Some(token) = tokens.next() {
            let data = token.strip_prefix('#').unwrap().parse::<u8>();
            let data = match data {
                Ok(data) => data,
                Err(err) => {
                    println!("Error: Invalid data: {}", err);
                    std::process::exit(1);
                }
            };
            data
        } else {
            0
        };

        let instruction = instructions::get_instruction(instruction, data);
        new_program.push(instruction);
    });
    let program = new_program;

    #[cfg(debug_assertions)]
    {
        println!("Program: {:?}", program);
    }

    /* This was just a test program to see some stuff */
    /* None of this does anything important */
    /* This program will be rewritten to work when I have time :) */
}
