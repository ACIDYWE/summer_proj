#[macro_use] pub mod helper_func;
pub mod checker;

use std::net::TcpStream;
use std::io::Read;
use std::ascii::AsciiExt;

pub use checker::*;

pub const TIME_ROUND: i64 = 30;   // min value 20
pub const TIME_ON_CHECK: i64 = 5; // min value 5
pub const FLAGS_COUNT: usize = 3; // min value 1

#[derive(Debug)]
pub enum ReadlineError {
    NotAscii,
    ConnectionLost,
}

pub trait ReadlineForTcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError>;
}

impl ReadlineForTcpStream for TcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError> {      //reading until new line
        let mut c: [u8;1] = [0;1];
        let mut readen = 0;
        self.read_exact(&mut c).unwrap();
        while c[0] != 10 {
            if !c[0].is_ascii() {return Err(ReadlineError::NotAscii);}
            readen+=1;
            (*buf).push(c[0] as char);
            match self.read_exact(&mut c) {
                Ok(_) => (),
                Err(_) => return Err(ReadlineError::ConnectionLost)
            }
        }
        Ok(readen)
    }
}
