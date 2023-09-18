#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug)]
pub enum Statement {
    Label {
        name: String,
        line: usize,
        col: usize,
    },
    Instruction {
        literal: String,
        operand: Operand,
        line: usize,
        col: usize,
    },
    Variable {
        name: String,
        value: Operand,
        line: usize,
        col: usize,
    },
}

#[derive(Debug)]
pub enum Operand {
    Expression {
        literal: String,
        line: usize,
        col: usize,
    },
    Int {
        literal: u8,
        line: usize,
        col: usize,
    },
    None,
}

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String,
    pub address: u8,
}
