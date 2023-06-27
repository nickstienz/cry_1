#[derive(Debug)]
#[repr(u8)]
pub(crate) enum Instructions {
    LDA(u8) = 0x0,
    LDB(u8) = 0x1,
    ADD = 0x2,
    OUTPUT = 0x3,
    HLT = 0x4,
}

pub(crate) fn get_instruction(instruction: &str, data: u8) -> Instructions {
    match instruction.to_uppercase().as_str() {
        "LDA" => Instructions::LDA(data),
        "LDB" => Instructions::LDB(data),
        "ADD" => Instructions::ADD,
        "OUTPUT" => Instructions::OUTPUT,
        "HLT" => Instructions::HLT,
        _ => {
            println!("Error: Unknown instruction: {}", instruction);
            std::process::exit(1);
        }
    }
}
