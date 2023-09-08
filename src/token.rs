#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label(String),
    Instruction(String),
    IntegerLiteral(u8),
    Error(String),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}