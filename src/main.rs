mod instructions;
use instructions::*;

fn main() {
    // This will be the assembler for the CRY-1 Computer so buckle up and enjoy your ride to hell!
    println!("=== CRY-1 Assembler v0.1.0 by Nicholas Stienz ===");

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

    let mut program: Vec<String> = Vec::with_capacity(contents.lines().count());
    for line in contents.lines() {
        program.push(line.to_string().trim().to_string());
    }

    #[cfg(debug_assertions)]
    {
        println!("Program In: {:?}\n", program);
    }

    program.retain(|line| !line.is_empty() && !line.starts_with(';'));
    for line in program.iter_mut() {
        if let Some(index) = line.find(';') {
            *line = line[..index].to_string().trim().to_string();
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Program Processed (1): {:?}\n", program);
    }

    // program: Vec<String> can be used now.

    Instruction::create_pai_data(); // This needs to happen before the program is parsed.
}
