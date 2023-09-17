#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    LabelStatement(Label),
    InstructionStatement(Instruction),
    VariableStatement(Variable),
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Operand,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub operand: Operand,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum Operand {
    Expression(String, usize, usize),
    Int(u8, usize, usize),
    None,
}
