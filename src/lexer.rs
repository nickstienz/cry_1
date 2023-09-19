use super::token::*;

pub struct Lexer {
    contents: String,
    line: usize,
    col: usize,
    position: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Lexer {
            contents,
            line: 0,
            col: 0,
            position: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut program: Vec<Token> = Vec::new();
        while let Some(token) = self.next() {
            program.push(token);
        }
        program
    }

    fn next(&mut self) -> Option<Token> {
        while !self.is_end_of_file() {
            if self.clear_redundent_characters() && !self.peek_character().is_whitespace() {
                return Some(self.consume_token());
            }
        }
        None
    }

    fn is_end_of_file(&self) -> bool {
        self.position >= self.contents.len()
    }

    fn clear_redundent_characters(&mut self) -> bool {
        let current_character = self.peek_character();

        if current_character == '\n' {
            self.skip_character('\n');
        } else if current_character.is_whitespace() {
            self.skip_character(' ');
        } else if current_character == ';' {
            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character == '\n' {
                    return false;
                }
                self.position += 1;
                self.col += 1;
            }
        } else {
            return false;
        }

        return true;
    }

    fn peek_character(&self) -> char {
        self.contents.chars().nth(self.position).unwrap()
    }

    fn skip_character(&mut self, target: char) {
        while let Some(current_character) = self.contents.chars().nth(self.position) {
            if current_character != target {
                break;
            }

            if current_character == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            self.position += 1;
        }
    }

    fn consume_token(&mut self) -> Token {
        let current_character = self.contents.chars().nth(self.position).unwrap();
        self.position += 1;
        self.col += 1;

        if current_character.is_alphabetic()
            || current_character == '_'
            || current_character == ':'
            || current_character == '.'
        {
            let start = self.col;
            let mut lexeme = String::new();
            lexeme.push(current_character);

            while let Some(next_character) = self.contents.chars().nth(self.position) {
                if next_character.is_alphabetic()
                    || next_character == '_'
                    || next_character == ':'
                    || next_character == '.'
                {
                    lexeme.push(next_character);
                    self.position += 1;
                    self.col += 1;
                } else {
                    break;
                }
            }

            if lexeme.starts_with('.') && !lexeme.ends_with(':') {
                Token {
                    token_type: TokenType::Variable(lexeme),
                    line: self.line,
                    col: start,
                }
            } else if !lexeme.starts_with('.') && lexeme.ends_with(':') {
                Token {
                    token_type: TokenType::Label(lexeme),
                    line: self.line,
                    col: start,
                }
            } else {
                if Token::is_instruction(&lexeme) {
                    Token {
                        token_type: TokenType::Instruction(lexeme),
                        line: self.line,
                        col: start,
                    }
                } else {
                    Token {
                        token_type: TokenType::Expression(lexeme),
                        line: self.line,
                        col: start,
                    }
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
