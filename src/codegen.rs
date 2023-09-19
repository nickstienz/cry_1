use core::panic;

use crate::{ast, token};

pub struct Codegen {
    pub program: ast::Program,
}

impl Codegen {
    pub fn new(program: ast::Program) -> Self {
        Self { program }
    }

    pub fn generate(&mut self) -> Vec<String> {
        let mut output = Vec::with_capacity(16);

        let mut statements = self.program.statements.iter();
        let symbols = &self.program.symbols;

        while let Some(ast_type) = statements.next() {
            match ast_type {
                ast::Statement::Label {
                    name: _,
                    line: _,
                    col: _,
                } => {
                    continue;
                }
                ast::Statement::Instruction {
                    literal,
                    operand,
                    line: _,
                    col: _,
                } => {
                    let binary = token::Token::to_binary(literal);
                    let mut op = String::new();
                    match operand {
                        ast::Operand::Expression {
                            literal,
                            line: _,
                            col: _,
                        } => {
                            for symbol in symbols {
                                let id = &symbol.identifier;
                                if id == literal {
                                    op = token::Token::to_binary(&symbol.address.to_string())
                                }
                            }
                            panic!("Could not match symbol in expression!");
                        }
                        ast::Operand::Int {
                            literal,
                            line: _,
                            col: _,
                        } => {
                            for symbol in symbols {
                                let id = &symbol.identifier;
                                if id == &literal.to_string() {
                                    op = token::Token::to_binary(&symbol.address.to_string())
                                }
                            }
                            panic!("Could not match symbol in int!");
                        }
                        ast::Operand::None => {}
                    };
                    output.push(format!("{}{}", binary, op));
                }
                _ => panic!("Codegen: How did we get here?"),
            }
        }

        output
    }
}
