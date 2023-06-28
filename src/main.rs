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

    // Read the file and handle the common errors using a match statement
    let contents = match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        // Handle common errors
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
        println!("Program Processed (1): {:?}\n", program);
        println!("Start Index: {}\n", start_index);
    }

    for i in 0..program.len() {
        let line = &mut program[i];
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 2 {
            let opcode = Instruction::opcode_from_string(tokens[0]);
            *line = format!("{}", opcode << 4);
        } else {
            let opcode = Instruction::opcode_from_string(tokens[0]);
            let operand = tokens[1].strip_prefix('#').unwrap().parse::<u8>().unwrap();
            *line = format!("{} #{}", opcode, operand);
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Program Processed (2): {:?}\n", program);
    }

    let mut new_elements = Vec::new();

    for i in 0..program.len() {
        if let Some(index) = program[i].find('#') {
            let (opcode, operand) = {
                let (opcode, operand) = program[i].split_at(index);
                (opcode, operand.to_string())
            };
            new_elements.push(operand[1..].to_string());
            let operand_index = program.len() - 1 + new_elements.len();
            program[i] = format!("{} {}", opcode, operand_index);
        }
    }

    program.extend(new_elements);

    #[cfg(debug_assertions)]
    {
        println!("Program Processed (3): {:?}\n", program);
    }

    for line in program.iter_mut() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let opcode = tokens[0].parse::<u8>().unwrap();
        if tokens.len() == 1 {
            *line = format!("{:08b}", opcode);
            continue;
        }
        let operand = tokens[1].parse::<u8>().unwrap();
        *line = format!("{:04b}{:04b}", opcode, operand);
    }

    #[cfg(debug_assertions)]
    {
        println!("Program Complete: {:#?}", program);
    }

    let mut output = String::new();
    for i in 0..program.len() {
        output.push_str(&program[i]);
        if i != program.len() - 1 {
            output.push('\n');
        }
    }

    println!("Output:\n{}", output);
}
