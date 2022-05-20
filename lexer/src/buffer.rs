use std::{error, fs::File, io::{BufReader, Error as IoError, Read}};

use thiserror::Error;
use anyhow::Error as AnyhowError;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;

const BUFFER_SIZE: usize = 2048;
const TOTAL_SIZE: usize = 4096;


// So, I need to read the data from a file into a buffer by buffer. 
#[derive(Debug)]
pub struct Buffer {
    file: File,
    buffer: [u8;TOTAL_SIZE],
    index: usize,
    fence: usize,
    eof: usize,
    next_char: Option<char>
}

#[derive(Error,Debug)]
pub enum BufferError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error("Parsed token is too big, greater than 4096 bytes")]
    RollBackError,
    #[error("EOF")]
    EndOfFile
}

impl Buffer {
    pub fn new(mut file: File) -> Result<Self, BufferError> {
        let mut buffer = [0;TOTAL_SIZE];
        let bytes_read = file.read(&mut buffer[0..BUFFER_SIZE])?;
        if bytes_read == 0 {
            return Err(BufferError::EndOfFile);
        }
        let next_char = buffer[0] as char;
        Ok(Buffer {
            file,
            buffer, 
            index: 0,
            fence: 0,
            eof: bytes_read,
            next_char: Some(next_char)
        })
    } 

    pub fn rollback(&mut self) -> Result<(), BufferError> {
        if self.index == self.fence {
            return Err(BufferError::RollBackError);
        }
        self.index -= 1;
        self.index = self.index % TOTAL_SIZE;

        Ok(())        
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.index == self.eof {
            return None;
        }

        return Some(self.buffer[self.index] as char);
    }

    pub fn get_next_char(&mut self) -> Result<char, BufferError> {
        // get character 
        let next_ch = self.buffer[self.index];

        // advance the cursor to the next element; 
        self.index += 1;
        self.index = self.index % TOTAL_SIZE; 
        if self.index % BUFFER_SIZE == 0 {
            let start_index = self.index; 
            let end_index = self.index + BUFFER_SIZE - 1;
            let buffer = &mut self.buffer[start_index..end_index];
            let bytes_read = self.file.read(buffer)?;
            self.fence = (self.index + BUFFER_SIZE) % TOTAL_SIZE;
            self.eof = self.index + bytes_read;
        }

        Ok(next_ch as char)
    }
}