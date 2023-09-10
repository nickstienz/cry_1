#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label(String),
    Instruction(String),
    IntegerLiteral(u8),
    Error(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn to_binary(&self) -> String {
        match &self.token_type {
            TokenType::Instruction(instr) => {
                match instr.as_str() {
                    "hlt" => String::from("0000"),
                    "lda" => String::from("0001"),
                    "add" => String::from("0010"),
                    "out" => String::from("0011"),
                    _ => panic!("Instruction not found: {}", instr),
                }
            },
            TokenType::IntegerLiteral(num) => {
                num.to_string()
            }
            _ => panic!("Token can not be converted to binary: {:?}", self),
        }
    }
}