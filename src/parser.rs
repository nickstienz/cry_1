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

        while let Some(token) = tokens.next() {
            match &token.token_type {
                TokenType::Label(label) => {
                    statements.push(ast::Statement::LabelStatement(ast::Label {
                        name: label.to_owned(),
                        line: token.line,
                        col: token.col,
                    }));
                }
                TokenType::Instruction(name) => {
                    let next_token = tokens.next().expect("No token after instr!");
                    let operand = match &next_token.token_type {
                        TokenType::Expression(exp) => ast::Operand::Expression(
                            exp.to_owned(),
                            next_token.line,
                            next_token.col,
                        ),
                        TokenType::IntegerLiteral(int) => {
                            ast::Operand::Int(int.to_owned(), next_token.line, next_token.col)
                        }
                        _ => ast::Operand::None,
                    };
                    statements.push(ast::Statement::InstructionStatement(ast::Instruction {
                        name: name.to_owned(),
                        operand,
                        line: token.line,
                        col: token.col,
                    }))
                }
                TokenType::Variable(name) => {
                    let next_token = tokens.next().expect("No token after var!");
                    let value = match &next_token.token_type {
                        TokenType::IntegerLiteral(int) => {
                            ast::Operand::Int(int.to_owned(), next_token.line, next_token.col)
                        }
                        _ => panic!("No int found after var!"),
                    };
                    statements.push(ast::Statement::VariableStatement(ast::Variable {
                        name: name.to_owned(),
                        value,
                        line: token.line,
                        col: token.col,
                    }))
                }
                _ => panic!("Error in Parser token match!"),
            }
        }

        ast::Program { statements }
    }
}
