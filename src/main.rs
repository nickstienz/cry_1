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
                println!("File not found: {}", filename);
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("Permission denied: {}", filename);
                std::process::exit(1);
            }
            _ => {
                println!("Error reading file: {}", filename);
                std::process::exit(1);
            }
        },
    };

    println!("With text:\n{}", contents);
}
