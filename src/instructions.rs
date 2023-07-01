use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, Debug, EnumIter)]
#[allow(non_camel_case_types)]
pub enum OperandType {
    LABEL,
    REF_EOP,
    ERROR,
}

pub struct PaiPattern {
    pub pattern: Regex,

    pub opcode_literal: String,

    pub operand_type: OperandType,
    pub operand_data: String,
}

pub struct PaiAssembler {
    pub patterns: Vec<PaiPattern>,
}

pub struct Instruction {
    pub opcode_literal: String,
    pub operand_literal: String,
}

impl Instruction {
    pub fn new(
        opcode_literal: String,
        operand_type: OperandType,
        operand_data: String,
        operand_literal: String,
    ) -> Self {
        Self {
            opcode_literal,
            operand_literal,
        }
    }

    pub fn operand_literal_to_enum(operand_literal: &str) -> OperandType {
        for operand in OperandType::iter() {
            if operand.to_string().to_lowercase() == operand_literal.to_lowercase() {
                return operand;
            }
        }
        return OperandType::ERROR;
    }

    pub fn create_pai_data() {
        // PAI = Patterns and Instructions
        const FILE_LOCATION: &str = "./pai.txt";

        let mut contents = match std::fs::read_to_string(FILE_LOCATION) {
            Ok(contents) => contents.replace("\n\n", "\n"),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("Error: File {} not found", FILE_LOCATION);
                    std::process::exit(1);
                }
                std::io::ErrorKind::PermissionDenied => {
                    println!("Error: Permission denied for file {}", FILE_LOCATION);
                    std::process::exit(1);
                }
                _ => {
                    println!(
                        "Something went wrong reading file {}: {}",
                        FILE_LOCATION, error
                    );
                    std::process::exit(1);
                }
            },
        };

        let patterns_start = contents
            .find("=== Patterns ===")
            .expect("Could not find patterns");
        let contents = contents
            .chars()
            .skip(patterns_start + "== Patterns ===".len() + 2)
            .collect::<String>();

        let split_contents: Vec<&str> = contents.split("=== Instructions ===").collect();
        let binding = split_contents[0].trim().lines().collect::<Vec<&str>>();
        let patterns = binding.windows(2).step_by(2);
        let instructions = split_contents[1].trim();

        for patten in patterns {
            println!("{:?}", patten);
        }
    }
}
