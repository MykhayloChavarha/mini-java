use std::{collections::VecDeque, num::ParseIntError, ops::Add, path::{self, Path, PathBuf}, str::FromStr, string::ParseError};

use dfa::DfaState;
use phf::{Map, phf_set}; 
use phf::phf_map;

use thiserror::Error;

mod dfa;


const BUFFER_SIZE: usize = 4096;

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

static BRACKET: phf::Map<&'static str, Token> = phf_map! {
    "{" => Token::LBrace, 
    "}" => Token::RBrace, 
    "[" => Token::LBrack, 
    "]" => Token::RBrack, 
    "(" => Token::LParen, 
    ")" => Token::RParen, 
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),

    Zero,
    Int,
    Number(u32),

    Boolean, 
    True, 
    False, 

    String, // <--- TODO

    Class, 
    Public, 
    Static, 
    Main, 
    
    Extends, 
    If, 
    Length,
    Else, 
    While,
    New, 
    This, 
    Void,
    Return, 

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,

    Semicolon,
    Comma, 
    Plus,
    Multiply,
    Minus,
    Divide,

    Assignment,
    Equals,

    Dot,
    Not,

    GreaterThan,
    GreaterThanEquals,
    LessThan, 
    LessThanEquals,

    BitwiseAnd,
    LogicalAnd,
    BitwiseOr,
    LogicalOr,

    NotEquals,
    Colon,

    System, 
    Out,
    Println,

    EOF
}

#[derive(Error, Debug, PartialEq)]
pub enum LexerError {
    #[error("Invalid token: {self}")]
    InvalidToken(String),
    #[error(transparent)]
    IntParseError(#[from] ParseIntError),
    #[error("EOF")]
    EndOfFile
}

#[derive(Debug)]
struct Buffer {
    buffer: Vec<u8>,
    buff_size: usize, 
    cursor: usize,
}

impl Buffer {
    pub fn new(buffer: Vec<u8>) -> Self {
        let lenght = buffer.len();
        Buffer {
            buffer,
            buff_size: lenght,
            cursor: 0
        }
    } 
    pub fn advance(&mut self) {
        self.cursor += 1; 
    }

    pub fn rollback(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1
        };
    }
    pub fn get_char(&mut self ) -> Option<char> {
        if self.cursor >= self.buff_size {
            return None
        } else {
            return Some(self.buffer[self.cursor] as char);
        }
    }
}
pub struct Lexer {
    dfa_states: VecDeque<DfaState>, 
    current_state: DfaState,
    lexeme: String, 
    buffer: Buffer
}

impl Lexer {
    pub fn new(path: PathBuf) -> Self {
        let buffer = std::fs::read(path).unwrap();
        let mut dfa_states = VecDeque::new();
        let current_state = DfaState::Start;
        dfa_states.push_back(DfaState::Error);

        Lexer {
            dfa_states,
            current_state,
            lexeme: "".to_string(),
            buffer: Buffer::new(buffer)
        }
    }

    fn advance(&mut self) {
        self.dfa_states.push_back(self.current_state.clone());
        if let Some(next_char) = self.buffer.get_char() {
            self.current_state = self.current_state.next_state(next_char); 
            if self.current_state != DfaState::Exit {
                self.lexeme.push(next_char as char); 
                self.buffer.advance();
            }
        } else {
            self.current_state = DfaState::Exit;
        }
    }

    fn set_accepting(&mut self) {
        self.dfa_states.clear();
    }

    pub fn get_next_token(&mut self) -> Result<Token,LexerError> { 
        self.lexeme = String::from("");
        self.dfa_states.push_back(DfaState::Error);
        self.current_state = DfaState::Start;
        loop {
            match &self.current_state {
                DfaState::Start => {
                    self.advance();
                },
                DfaState::Identifier => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Zero => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Number => {
                    self.set_accepting(); 
                    self.advance()
                }
                DfaState::Bracket => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Plus => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Minus => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Multiply => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Div => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Assignment => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Equals => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::LessThan => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::LessThanEquals => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::GreaterThan => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::GreaterThanEquals => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Not => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::NotEquals => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::BitwiseAnd => {
                    self.advance()},
                DfaState::LogicalAnd => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::BitwiseOr => {
                    self.advance()
                },
                DfaState::LogicalOr => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Dot => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Comma => {
                    self.set_accepting(); 
                    self.advance()
                },
                DfaState::Semicolon => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Colon => {
                    self.set_accepting(); 
                    self.advance()},
                DfaState::Exit => {
                    break;
                },
                DfaState::Error => todo!(),
                
            }
        }

        self.generate_token()

    }

    fn generate_token(&mut self) -> Result<Token, LexerError> {
        loop {
            if let Some(state) = self.dfa_states.pop_back() {
                match state {
                    DfaState::Start => {
                        self.buffer.rollback();
                    },
                    DfaState::Identifier => {
                        let id = self.lexeme.trim();
                        if let Some(entry) = KEYWORD.get(id) {
                            return Ok(entry.clone());
                        } 
                        return Ok(Token::Identifier(id.to_string()));
                    },
                    DfaState::Zero => {
                        return Ok(Token::Number(0));
                    },
                    DfaState::Number => {
                        let number = self.lexeme.trim();
                        return Ok(Token::Number(number.parse::<u32>()?));
                    },
                    DfaState::Bracket => {
                        let bracket_id = self.lexeme.trim();
                        if let Some(br) = BRACKET.get(bracket_id) {
                            return Ok(br.clone());
                        }
                        return Err(LexerError::InvalidToken(self.lexeme.clone()));
                    },
                    DfaState::Plus => {
                        return Ok(Token::Plus);
                    },
                    DfaState::Minus => {
                        return Ok(Token::Minus);
                    },
                    DfaState::Multiply => {
                        return Ok(Token::Multiply)
                    },
                    DfaState::Div => {
                        return Ok(Token::Divide)
                    },
                    DfaState::Assignment => {
                        return Ok(Token::Assignment)
                    },
                    DfaState::Equals => {
                        return Ok(Token::Equals)
                    },
                    DfaState::LessThan => {
                        return Ok(Token::LessThan)
                    },
                    DfaState::LessThanEquals => {
                        return Ok(Token::LessThanEquals)
                    },
                    DfaState::GreaterThan => {
                        return Ok(Token::GreaterThan)
                    },
                    DfaState::GreaterThanEquals => {
                        return Ok(Token::GreaterThanEquals)
                    },
                    DfaState::Not => {
                        return Ok(Token::Not)
                    },
                    DfaState::NotEquals => {
                        return Ok(Token::NotEquals)
                    },
                    DfaState::BitwiseAnd => {
                        return Ok(Token::BitwiseAnd)
                    },
                    DfaState::LogicalAnd => {
                        return Ok(Token::LogicalAnd)
                    },
                    DfaState::BitwiseOr => {
                        return Ok(Token::BitwiseOr)
                    },
                    DfaState::LogicalOr => {
                        return Ok(Token::LogicalOr)
                    },
                    DfaState::Dot => {
                        return Ok(Token::Dot)
                    },
                    DfaState::Comma => {
                        return Ok(Token::Comma)
                    },
                    DfaState::Semicolon => {
                        return Ok(Token::Semicolon)
                    },
                    DfaState::Colon => {
                        return Ok(Token::Colon)
                    },
                    DfaState::Exit => todo!(),
                    DfaState::Error => {
                        return Err(LexerError::InvalidToken(self.lexeme.clone()))
                    },
                    
                }
            }
            
        }


    } 
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};
    use anyhow::{Error, Result};

    use crate::{Lexer, LexerError, Token};

    #[test]
    fn load_program() -> Result<()> { 
        Ok(())
    }

    #[test]
    fn should_be_empty_string() -> Result<()> {
        let path = "../sample/Space.java";
        let mut lexer = Lexer::new(PathBuf::from(path));
        let error = lexer.get_next_token().expect_err("error occured");
        assert_eq!(error, LexerError::InvalidToken(" ".to_string()));
        Ok(())
    }
    #[test]
    fn should_be_newline() -> Result<()> {
        let path = "../sample/NewLine.java";
        let mut lexer = Lexer::new(PathBuf::from(path));
        let error = lexer.get_next_token().expect_err("");

        assert_eq!(error, LexerError::InvalidToken(" \n ".to_string()));
        Ok(())
    }

    #[test]
    fn should_be_valid_tokens() -> Result<()> {
        let path = "../sample/IdentifierPublic.java";
        let mut lexer = Lexer::new(PathBuf::from(path));

        let token = lexer.get_next_token().unwrap();
        if let Token::Identifier(token_str) =  token {
            assert_eq!(token_str, "public");
        } 

        let token = lexer.get_next_token();
        if let Ok(Token::Identifier(token_str)) = token {
            assert_eq!(token_str, "static");
        } 
        Ok(())   
    }
    #[test]
    fn should_be_invalid_token() -> Result<()> {
        // let path = "../sample/IdentifierInvalid.java";
        // let mut lexer = Lexer::new(PathBuf::from(path));

        // let token = lexer.get_next_token();
        // if let Err(invalid_token) =  token {
        //     assert_eq!(invalid_token, LexerError::InvalidToken("1".to_string()));
        // } 
        Ok(())   
    }

    #[test]
    fn should_be_valid_int32() -> Result<()> {
        let path = "../sample/Integer.java";
        let mut lexer = Lexer::new(PathBuf::from(path));

        let token = lexer.get_next_token()?;
        assert_eq!(Token::Number(123), token);

        let token2 = lexer.get_next_token()?;
        assert_eq!(Token::Number(0), token2);

        let token3 = lexer.get_next_token()?;
        assert_eq!(Token::Number(535), token3);
        Ok(())
    }
    #[test]
    fn should_be_valid_sequence() -> Result<()> {
        let path = "../sample/Sequence.java";
        let mut lexer = Lexer::new(PathBuf::from(path));

        let token = lexer.get_next_token()?;
        assert_eq!(Token::Number(0), token);

        let token = lexer.get_next_token()?;
        assert_eq!(Token::Number(321),token);
        
        let token3 = lexer.get_next_token()?;
        assert_eq!(Token::Minus, token3);

        let token4 = lexer.get_next_token()?;
        assert_eq!(Token::Identifier("string".to_string()), token4);

        Ok(())
    }

    #[test]
    fn should_be_valid_factorial_program() -> Result<()> {
        let path = "../sample/Factorial.java";
        let mut lexer = Lexer::new(PathBuf::from(path));

        let mut token = lexer.get_next_token()?;
        assert_eq!(Token::Class, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Identifier("Factorial".to_string()), token);
        token = lexer.get_next_token()?;
        assert_eq!(Token::LBrace, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Public, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Static, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Void, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Main, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::LParen, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::String, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::LBrack, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::RBrack, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Identifier("a".to_string()), token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::RParen, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::LBrace, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::System, token);

        token = lexer.get_next_token()?;
        assert_eq!(Token::Dot, token);

        Ok(())
    }
}
