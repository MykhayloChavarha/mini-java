use std::{borrow::Borrow, fs::File, io::Read, num::{ParseFloatError, ParseIntError}};
use buffer::{Buffer, BufferError};
use phf::phf_map;
use thiserror::Error;
use anyhow::Error;

mod buffer;

// const BUFFER_SIZE: usize = 4096;
#[derive(Clone)]
pub enum Token {
    Identifier(String),
    Number(u32),

    Boolean, 
    True, 
    False, 
    Int,
    String,
    Class, 
    Public, 
    Static, 
    Main, 
    Extends, 
    If, 
    Else, 
    While, 
    New, 
    This, 
    Void,
    Return, 
    Length,
    System, 
    Out,
    Println,

    // parentesies 
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    
    // 
    Plus,
    Multiply,
    Minus,
    Divide,

    // =, ==, 
    Assignment,
    Equals,

    // !, !=, 
    Not,
    NotEquals,

    // >, <, >=, <=
    GreaterThan,
    GreaterThanEquals,
    LessThan, 
    LessThanEquals,

    // &,|,&&,||
    BitwiseAnd,
    LogicalAnd,
    BitwiseOr,
    LogicalOr,

    // :, ;, .
    Colon,
    Semicolon, 
    Dot,

    Dummy
}
#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error(transparent)]
    IntParseError(#[from] ParseIntError),
    #[error(transparent)]
    FloatParseError(#[from] ParseFloatError),
    #[error(transparent)]
    FileError(#[from] BufferError),
    #[error("End of file is reached")]
    EndOfFile,
    #[error("Unknown.")]
    Unknown,
}

pub struct Tokenizer<R: Read> {
    buffer: Buffer<R, 4096>,  
}

impl<R: Read> Tokenizer<R> {
    pub fn new(file: R) -> Result<Self, BufferError> {
        let buffer = Buffer::new(file)?;

        Ok(Tokenizer{
            buffer
        })
    }

    pub fn get_next_token(&mut self) -> Result<Token, TokenizerError> {
        self.trim()?;
        if let Some(next_char) = self.buffer.peek() { 
            match next_char {
                'a'..='z'|'A'..='Z' => {
                    return self.tokenize_identifier();
                }, 
                '0'..='9' => {
                    return self.tokenize_number(); 
                },
                '{'|'}'|'['|']'|'('|')' => {
                    return self.tokenize_bracket();
                },
                '+'|'-'|'*'|'/' => {
                    return self.tokenize_num_operator();
                },
                '=' => {
                    self.buffer.get_next_char()?; 
                    if let Some('=') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::Equals)
                    } 
                    return Ok(Token::Assignment);
                },
                '!' => {
                    self.buffer.get_next_char()?;
                    if let Some('=') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::NotEquals);
                    }
                    return Ok(Token::Not);
                }, 
                '>' => {
                    self.buffer.get_next_char()?;
                    if let Some('=') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::GreaterThanEquals);
                    }
                    return Ok(Token::GreaterThan);
                },
                '<' => {
                    self.buffer.get_next_char()?;
                    if let Some('=') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::LessThanEquals);
                    }
                    return Ok(Token::LessThan);
                },
                '&' => {
                    self.buffer.get_next_char()?;
                    if let Some('&') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::LogicalAnd);
                    }
                    return Ok(Token::BitwiseAnd);
                },
                '|' => {
                    self.buffer.get_next_char()?;
                    if let Some('|') = self.buffer.peek() {
                        self.buffer.get_next_char()?;
                        return Ok(Token::LogicalOr);
                    }
                    return Ok(Token::LogicalAnd);
                },
                ':' => {
                    self.buffer.get_next_char()?;
                    return Ok(Token::Colon)
                }
                ';' => {
                    self.buffer.get_next_char()?;
                    return Ok(Token::Semicolon)
                },
                '.' => {
                    self.buffer.get_next_char()?;
                    return Ok(Token::Dot)
                }, 
                _ => {
                }
            }
        }

        Err(TokenizerError::EndOfFile)
    } 

    fn tokenize_identifier(&mut self) -> Result<Token, TokenizerError> {
        let mut lexeme = String::new();
        lexeme.reserve(50);
        while let Some(char) = self.buffer.peek() {
            match char {
                'a'..='z'|'A'..='Z'|'_' => {
                    lexeme.push(self.buffer.get_next_char()?);
                },
                _ => {
                    break;
                }
            }
        }
        if let Some(identifier) = KEYWORD.get(&lexeme) {
            return Ok(identifier.clone());
        }
        Ok(Token::Identifier(lexeme))
    }

    fn tokenize_number(&mut self) -> Result<Token, TokenizerError> {
        let mut lexeme = String::new();
        lexeme.reserve(10);
        while let Some(number) = self.buffer.peek() {
            match number {
                '0'..='9' => {
                    lexeme.push(number);
                }
                _ => {
                    break;
                }
            }
        }
        let number = lexeme.parse::<u32>().unwrap();
        return Ok(Token::Number(number))
    }

    fn tokenize_bracket(&mut self) -> Result<Token, TokenizerError> {
        let next_char = self.buffer.get_next_char()?;
        if let Some(token) = BRACKET.get(&next_char) {
            return Ok((*token).clone());
        }
        Err(TokenizerError::Unknown)
    }

    fn tokenize_num_operator(&mut self) -> Result<Token, TokenizerError> {
        let next_char = self.buffer.get_next_char()?;
        if let Some(token) = NUMERICAL_OPERATOR.get(&next_char) {
            return Ok(token.clone());
        }
        Err(TokenizerError::Unknown)
    }

    // removes whitespaces;
    // TODO: Add comments too; 
    fn trim(&mut self) -> Result<(), TokenizerError> {
        let next_token = self.buffer.peek();
        loop {
            if let Some('\t'|' '|'\n') = self.buffer.peek() {
                self.buffer.get_next_char()?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

static KEYWORD: phf::Map<&'static str, Token> = phf_map! {
    "boolean" => Token::Boolean, 
    "true" => Token::True, 
    "false" => Token::False, 
    "int" => Token::Int,
    "String" => Token::String,
    "class" => Token::Class, 
    "public" => Token::Public, 
    "static" => Token::Static, 
    "main" => Token::Main, 
    "extends" => Token::Extends, 
    "if" => Token::If, 
    "else" => Token::Else, 
    "while" => Token::While, 
    "new" => Token::New, 
    "this" => Token::This, 
    "void" => Token::Void,
    "return" => Token::Return, 
    "length" => Token::Length,
    "System" => Token::System, 
    "out" => Token::Out,
    "println" => Token::Println
    // System.out.println
};

static BRACKET: phf::Map<char, Token> = phf_map! {
    '{' => Token::LBrace, 
    '}' => Token::RBrace, 
    '[' => Token::LBrack, 
    ']' => Token::RBrack, 
    '(' => Token::LParen, 
    ')' => Token::RParen, 
};

static NUMERICAL_OPERATOR: phf::Map<char, Token> = phf_map! {
    '-' => Token::Minus, 
    '+' => Token::Plus, 
    '/' => Token::Divide, 
    '*' => Token::Multiply, 
};


#[cfg(test)]
mod tests {

    use anyhow::Result;
    #[test]
    fn should_return_true() -> Result<()> {
        Ok(())
    }
}