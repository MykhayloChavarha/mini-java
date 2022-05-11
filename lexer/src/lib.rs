use std::{collections::VecDeque, ops::Add, path::{self, Path, PathBuf}};
use thiserror::Error;
mod lexical_analyzer;


const BUFFER_SIZE: usize = 4096;
// define dfa state, at this moment let it be in three different states. 
// initial, identifier and error. 
#[derive(Clone)]
enum DfaState {
    S0,
    Identifier,
    Minus,
    Int32,
    Error,
    Out
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Minus,
    Int32(i32),
    EOF
}

#[derive(Error, Debug, PartialEq)]
pub enum LexerError {
    #[error("Invalid token: {self}")]
    InvalidToken(String),
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
    // later it might be a good idea to wrap the api for t his into some struct 
    // with functions: advance, rollback. get_char
    buffer: Buffer
}

impl Lexer {
    pub fn new(path: PathBuf) -> Self {
        let buffer = std::fs::read(path).unwrap();
        let index = 0; 
        let mut dfa_states = VecDeque::new();
        let current_state = DfaState::S0;
        dfa_states.push_back(DfaState::Error);

        Lexer {
            dfa_states,
            current_state,
            buffer: Buffer::new(buffer)
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token,LexerError> { 
        let mut lexeme = String::from("");
        self.dfa_states.push_back(DfaState::Error);
        self.current_state = DfaState::S0;
        loop {
            match &self.current_state {
                DfaState::S0 => { 
                    self.dfa_states.push_back(self.current_state.clone());
                    if let Some(next_char) = self.buffer.get_char() {
                        lexeme.push(next_char as char); 
                        match next_char {
                            'A'..='Z'|'a'..='z' => {
                                self.current_state = DfaState::Identifier;
                            },
                            ' '|'\n' => {
                                self.current_state = DfaState::S0;
                            },
                            _ => {
                                self.current_state = DfaState::Out;
                            }
                        }  
                    } else {
                        self.current_state = DfaState::Out;
                    }
                    self.buffer.advance(); 
                },
                DfaState::Identifier => {
                    self.dfa_states.clear(); 
                    self.dfa_states.push_back(DfaState::Identifier);

                    if let Some(next_char) = self.buffer.get_char() {
                        lexeme.push(next_char);
                        match next_char {
                            'A'..='Z'|'a'..='z'|'_'|'0'..='9' => {
                                self.current_state = DfaState::Identifier;
                            },
                            _ => {
                                self.current_state = DfaState::Out;
                            }
                        }
                    } else {
                        self.current_state = DfaState::Out;
                    } 
                    self.buffer.advance();    
                },
                DfaState::Minus => {
                    self.dfa_states.clear();
                    self.dfa_states.push_back(DfaState::Minus);
                },
                DfaState::Int32 => {
                }
                DfaState::Error => todo!(),
                DfaState::Out => {
                    let mut state: DfaState; 
                    loop {
                        state = self.dfa_states.pop_back().unwrap();
                        match state {
                            DfaState::S0 => {
                                self.buffer.rollback();
                            },
                            DfaState::Identifier => {
                                return Ok(Token::Ident(lexeme.trim().to_string()));
                            },
                            _ => {
                                return Err(LexerError::InvalidToken(lexeme));
                            }
                        }                 
                    }   
                },
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
        if let Token::Ident(token_str) =  token {
            assert_eq!(token_str, "public");
        } 

        let token = lexer.get_next_token();
        if let Ok(Token::Ident(token_str)) = token {
            assert_eq!(token_str, "static");
        } 
        Ok(())   
    }
    #[test]
    fn should_be_invalid_token() -> Result<()> {
        let path = "../sample/IdentifierInvalid.java";
        let mut lexer = Lexer::new(PathBuf::from(path));

        let token = lexer.get_next_token();
        if let Err(invalid_token) =  token {
            assert_eq!(invalid_token, LexerError::InvalidToken("1".to_string()));
        } 
        Ok(())   
    }
}
