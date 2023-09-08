use super::token::*;

pub struct Lexer {
    contents: String,
    line: usize,
    col: usize,
    position: usize,
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

    pub fn next(&mut self) -> Token {
        if self.position >= self.contents.len() {
            return Token {
                token_type: TokenType::EOF,
                line: self.line,
                col: self.col,
            };
        }

        let current_character = self.contents.chars().nth(self.position).unwrap();
        self.position += 1;
        self.col += 1;

        if current_character == '\n' {
            self.line += 1;
            self.col = 0;
            Token {
                token_type: TokenType::Linebreak,
                line: self.line,
                col: self.col,
            }
        } else if current_character.is_whitespace() {
            let start = self.col;
            while let Some(next_char) = self.contents.chars().nth(self.position) {
                if next_char.is_whitespace() {
                    self.position += 1;
                    self.col += 1;
                } else {
                    break;
                }
            }
            Token {
                token_type: TokenType::Whitespace,
                line: self.line,
                col: start,
            }
        } else if current_character == ';' {
            let start = self.col;
            let mut comment = String::new();
            comment.push(current_character);

            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character == '\n' {
                    break;
                }
                comment.push(next_character);
                self.position += 1;
                self.col += 1;
            }

            Token {
                token_type: TokenType::Comment(comment),
                line: self.line,
                col: start,
            }
        } else if current_character.is_alphabetic()
            || current_character == '_'
            || current_character == ':'
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
                token_type: TokenType::IntegerLiteral(lexeme),
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
