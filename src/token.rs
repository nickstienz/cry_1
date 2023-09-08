#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label(String),
    Instruction(String),
    IntegerLiteral(String),
    
    Comment(String),
    Whitespace,
    Linebreak,

    EOF,
    Error(String),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}