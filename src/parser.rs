use crate::{
    ast,
    token::{Token, TokenType},
};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> ast::Program {
        let mut tokens = self.tokens.iter();

        let mut statements: Vec<ast::Statement> = Vec::new();
        let mut symbols: Vec<ast::Symbol> = Vec::new();
        let mut instruction_pointer: u8 = 0;
        let mut variable_pointer: u8 = 15;

        while let Some(token) = tokens.next() {
            match &token.token_type {
                TokenType::Label(label) => {
                    symbols.push(ast::Symbol {
                        identifier: label.to_owned()[..label.len() - 1].to_string(),
                        address: instruction_pointer,
                    });
                    statements.push(ast::Statement::Label {
                        name: label.to_owned()[..label.len() - 1].to_string(),
                        line: token.line,
                        col: token.col,
                    });
                }
                TokenType::Instruction(name) => {
                    let next_token = tokens.next().expect("No token after instr!");
                    let operand = match &next_token.token_type {
                        TokenType::Expression(exp) => ast::Operand::Expression {
                            literal: exp.to_owned(),
                            line: next_token.line,
                            col: next_token.col,
                        },
                        TokenType::IntegerLiteral(int) => {
                            symbols.push(ast::Symbol {
                                identifier: int.to_string(),
                                address: variable_pointer,
                            });
                            variable_pointer -= 1;
                            ast::Operand::Int {
                                literal: int.to_owned(),
                                line: next_token.line,
                                col: next_token.col,
                            }
                        }
                        _ => ast::Operand::None,
                    };
                    instruction_pointer += 1;
                    statements.push(ast::Statement::Instruction {
                        literal: name.to_owned(),
                        operand,
                        line: token.line,
                        col: token.col,
                    })
                }
                TokenType::Variable(name) => {
                    let next_token = tokens.next().expect("No token after var!");
                    let value = match &next_token.token_type {
                        TokenType::IntegerLiteral(int) => ast::Operand::Int {
                            literal: int.to_owned(),
                            line: next_token.line,
                            col: next_token.col,
                        },
                        _ => panic!("No int found after var!"),
                    };
                    symbols.push(ast::Symbol {
                        identifier: name.to_owned()[1..].to_string(),
                        address: variable_pointer,
                    });
                    variable_pointer -= 1;
                    statements.push(ast::Statement::Variable {
                        name: name.to_owned()[1..].to_string(),
                        value,
                        line: token.line,
                        col: token.col,
                    })
                }
                _ => panic!("Error in Parser token match!"),
            }
        }

        ast::Program {
            statements,
            symbols,
        }
    }
}
