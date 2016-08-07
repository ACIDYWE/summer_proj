#[macro_use] pub mod helper_func;
pub mod checker;
pub mod db;

use std::net::TcpStream;
use std::io::Read;
use std::ascii::AsciiExt;

pub use checker::*;

pub const TIME_ROUND: i64 = 3*60; // min value 20
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
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError> {
        let mut c = [0u8];
        let mut readen = 0;
        loop {
            if let Err(_) = self.read_exact(&mut c) {
                return Err(ReadlineError::ConnectionLost);
            }
            if c[0] == 10 { break; }

            if !c[0].is_ascii() { return Err(ReadlineError::NotAscii); }
            buf.push(c[0] as char);
            readen += 1;
        }
        Ok(readen)
    }
}
