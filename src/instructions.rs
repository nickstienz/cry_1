use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, Debug, EnumIter)]
#[allow(non_camel_case_types)]
pub enum OperandType {
    LABEL,
    REF_EOP,
    NONE,
}

#[derive(Debug)]
pub struct Pai {
    pub pattern: Regex,

    pub opcode: String,

    pub operand_type: OperandType,
    pub operand_type_data: String,
}

pub struct Instruction {
    pub opcode_literal: String,
    pub operand_literal: String,
}

impl Instruction {
    pub fn _new(opcode_literal: String, operand_literal: String) -> Self {
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
        return OperandType::NONE;
    }

    pub fn create_pai_data() -> Vec<Pai> {
        // PAI = Patterns and Instructions
        const FILE_LOCATION: &str = "./pai.txt";

        let contents = match std::fs::read_to_string(FILE_LOCATION) {
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
        let binding = split_contents[1].trim().lines().collect::<Vec<&str>>();
        let binding = &binding
            .iter()
            .flat_map(|instruction| instruction.split_whitespace())
            .collect::<Vec<_>>();
        let instructions = binding.windows(2).step_by(2);

        let mut pai_vec: Vec<Pai> = Vec::with_capacity(patterns.len());

        for pattern in patterns {
            let regex = pattern[1];
            let instruction_template = pattern[0];

            let pattern_regex = Regex::new(regex).unwrap();
            let mut opcode = String::from("NONE");
            let mut operand_type = OperandType::NONE;
            let mut operand_type_data = String::from("NONE");

            for i in instructions.clone() {
                let ins = i[0];
                let code = i[1];

                let operand_type_regex = Regex::new(r"@(\w+)").unwrap();
                let operand_data_regex = Regex::new(r"@\w+:(\w+)").unwrap();

                if operand_type_regex.is_match(instruction_template) {
                    operand_type = Instruction::operand_literal_to_enum(
                        operand_type_regex
                            .captures(instruction_template)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str(),
                    );
                }

                if operand_data_regex.is_match(instruction_template) {
                    operand_type_data = String::from(
                        operand_data_regex
                            .captures(instruction_template)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str(),
                    );
                }

                let ins_regex = Regex::new(format!(r"{}( +|$)", ins).as_str()).unwrap();
                if ins_regex.is_match(instruction_template) {
                    opcode = code.to_string();
                    break;
                }
            }

            let new_pai = Pai {
                pattern: pattern_regex,
                opcode,
                operand_type,
                operand_type_data,
            };
            pai_vec.push(new_pai);
        }

        return pai_vec;
    }
}
