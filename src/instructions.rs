pub struct Instruction {
    pub opcode: u8,
    pub operand: u8,
}

impl Instruction {
    pub fn opcode_from_string(opcode: &str) -> u8 {
        match opcode.to_lowercase().trim() {
            "nop" => 0b0000,
            "sta" => 0b0001,
            "lda" => 0b0010,
            "ldi" => 0b0011,
            "add" => 0b0100,
            "sub" => 0b0101,
            "out" => 0b0110,
            "jmp" => 0b0111,
            "jc" => 0b1000,
            "jz" => 0b1001,
            "jnc" => 0b1010,
            "ldb" => 0b1011,
            "hlt" => 0b1111,
            _ => {
                println!("Error: Invalid opcode {}", opcode);
                std::process::exit(1);
            }
        }
    }
}
