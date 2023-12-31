use crate::instructions;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label(String),
    Instruction(String),
    IntegerLiteral(u8),
    Variable(String),
    Expression(String),
    Error(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

instructions!(
    hlt => 0000, // 0
    lda => 0001, // 1
    add => 0010, // 2
    sta => 0011, // 3
    sub => 0100, // 4
    jmp => 0101, // 5
    jz  => 0110, // 6
    out => 0111, // 7
);

#[macro_export]
macro_rules! instructions {
    (
        $($instr:ident => $binary:tt,)*
    ) => {
        impl Token {
            pub fn is_instruction(lexeme: &str) -> bool {
                match lexeme {
                    $(stringify!($instr) => true,)*
                    _ => false,
                }
            }

            pub fn to_binary(literal: &String) -> String {
                let literal = literal.as_str();
                if Token::is_instruction(literal) {
                    match literal {
                        $(stringify!($instr) => stringify!($binary).to_string(),)*
                        _ => panic!("Instruction not found: {}", literal),
                    }
                } else {
                    return format!("{:08b}", literal.parse::<u8>().unwrap());
                }
            }
        }
    };
}
