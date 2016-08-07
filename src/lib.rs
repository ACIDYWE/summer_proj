pub mod random;
pub mod client;
pub mod config;

use std::net::TcpStream;
use std::io::Read;
use std::ascii::AsciiExt;

pub trait ReadlineForTcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError>;
}

#[derive(Debug)]
pub enum ReadlineError {
    NotAscii,
    ConnectionLost,
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
