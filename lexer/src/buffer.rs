use std::{error, fs::File, io::{BufReader, Error as IoError, Read}};

use thiserror::Error;
use anyhow::Error as AnyhowError;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;


#[derive(Error,Debug)]
pub enum BufferError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error("Parsed token is too big, greater than 4096 bytes")]
    RollBackError,
    #[error("EOF")]
    EndOfFile
}


// 
#[derive(Debug)]
pub struct Buffer<R: Read, const N: usize> {
    input: R,
    half_buffer: usize,
    buffer: [u8;N],
    index: usize,
    fence: usize,
    eof: usize,
}

impl<R: Read, const N: usize> Buffer<R, N> {
    pub fn new(mut input: R) -> Result<Self, BufferError> {
        assert!(N % 2 == 0);
        let mut buffer = [0;N];
        let half_buffer = N/2;
        let bytes_read = input.read(&mut buffer[0..half_buffer])?;
        if bytes_read == 0 {
            return Err(BufferError::EndOfFile);
        }
        Ok(Buffer {
            input,
            half_buffer,
            buffer, 
            index: 0,
            fence: 0,
            eof: bytes_read
        })
    } 

    // issue rollback error when fence is reached. 
    pub fn rollback(&mut self) -> Result<(), BufferError> {
        if self.index == self.fence {
            return Err(BufferError::RollBackError);
        }

        if self.index == 0 {
            self.index = N - 1;
        } else {
            self.index -= 1;
        }
        self.index = self.index % N;
        

        Ok(())        
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.index == self.eof {
            return None;
        }

        return Some(self.buffer[self.index] as char);
    }

    pub fn get_next_char(&mut self) -> Result<char, BufferError> {
        if self.index == self.eof {
            return Err(BufferError::EndOfFile);
        }
        // get character 
        let next_ch = self.buffer[self.index];

        // advance the cursor to the next element; 
        self.index += 1;
        self.index = self.index % N; 
        if self.index % self.half_buffer == 0 {
            let start_index = self.index; 
            let end_index = self.index + self.half_buffer;
            let buffer = &mut self.buffer[start_index..end_index];
            let bytes_read = self.input.read(buffer)?;
            self.fence = (self.index + self.half_buffer) % N;
            self.eof = self.index + bytes_read;
        }

        Ok(next_ch as char)
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use crate::buffer::Buffer;

    #[test]
    fn should_return_true() -> Result<()> {

        let hello = ['h' as u8,'e' as u8,'l' as u8,'l' as u8,'o' as u8];

        let mut buffer = Buffer::<&[u8],4>::new(&hello)?;

        assert_eq!(buffer.peek(),Some('h'));
        assert_eq!(buffer.get_next_char()?,'h');
        assert_eq!(buffer.get_next_char()?,'e');

        assert_eq!(buffer.get_next_char()?,'l');
        assert_eq!(buffer.get_next_char()?,'l');

        assert_eq!(buffer.get_next_char()?,'o');
        buffer.get_next_char().expect_err("End of File ");

        buffer.rollback()?;
        buffer.rollback()?;
        buffer.rollback()?;
        buffer.rollback().expect_err("Fence");

        // assert_eq!(buffer.get_next_char()?,'o');
        Ok(())
    }
}
