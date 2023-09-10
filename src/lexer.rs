use super::token::*;

pub struct Lexer {
    contents: String,
    line: usize,
    col: usize,
    position: usize,
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut passed_checks = false;
        while passed_checks == false {
            if self.position >= self.contents.len() {
                return None;
            }
            passed_checks = self.clear_redundent_characters();
        }

        Some(self.consume_token())
    }
}

impl Lexer {
    pub fn new(contents: String) -> Lexer {
        Lexer {
            contents,
            line: 1,
            col: 0,
            position: 0,
        }
    }

    fn clear_redundent_characters(&mut self) -> bool {
        let mut passes_checks = true;

        let current_character = self.contents.chars().nth(self.position).unwrap();

        if current_character == '\n' {
            passes_checks = false;
            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character == '\n' {
                    self.line += 1;
                    self.position += 1;
                    self.col = 0;
                } else {
                    break;
                }
            }
        }

        let current_character = self.contents.chars().nth(self.position).unwrap();
        if current_character.is_whitespace() {
            passes_checks = false;
            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character.is_whitespace() {
                    self.position += 1;
                    self.col += 1;
                } else {
                    break;
                }
            }
        }

        let current_character = self.contents.chars().nth(self.position).unwrap();
        if current_character == ';' {
            passes_checks = false;
            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character == '\n' {
                    break;
                }
                self.position += 1;
                self.col += 1;
            }
        }

        passes_checks
    }

    fn consume_token(&mut self) -> Token {
        let current_character = self.contents.chars().nth(self.position).unwrap();
        self.position += 1;
        self.col += 1;

        if current_character.is_alphabetic() || current_character == '_' || current_character == ':'
        {
            let start = self.col;
            let mut lexeme = String::new();
            lexeme.push(current_character);

            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character.is_alphabetic() || next_character == '_' || next_character == ':'
                {
                    lexeme.push(next_character);
                    self.position += 1;
                    self.col += 1;
                } else {
                    break;
                }
            }

            if lexeme.ends_with(':') {
                Token {
                    token_type: TokenType::Label(lexeme),
                    line: self.line,
                    col: start,
                }
            } else {
                Token {
                    token_type: TokenType::Instruction(lexeme),
                    line: self.line,
                    col: start,
                }
            }
        } else if current_character.is_digit(10) {
            let start = self.col;
            let mut lexeme = String::new();
            lexeme.push(current_character);

            while let Some(next_char) = self.contents.chars().nth(self.position) {
                if next_char.is_digit(10) {
                    lexeme.push(next_char);
                    self.position += 1;
                    self.col += 1;
                } else {
                    break;
                }
            }

            Token {
                token_type: TokenType::IntegerLiteral(lexeme.parse::<u8>().unwrap()),
                line: self.line,
                col: start,
            }
        } else {
            Token {
                token_type: TokenType::Error(current_character.to_string()),
                line: self.line,
                col: self.col,
            }
        }
    }
}
