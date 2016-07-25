pub mod page;
pub mod pages;

use std::net::TcpStream;
use std::io::Read;

pub trait ReadlineForTcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize>;
}

pub enum Readline_error {
    NOT_ASCII,
    CONNECTION_LOST,
}


impl ReadlineForTcpStream for TcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize> {      //reading until new line
        let mut c: [u8;1] = [0;1];
        let mut readen = 0;
        self.read_exact(&mut c).unwrap();
        while c[0] != 10 {
            if !c[0].is_ascii() {
                return Err(Readline_error::NOT_ASCII);
            }
            readen+=1;
            (*buf).push(c[0] as char);
            match self.read_exact(&mut c) {
                Err(_) => return Err(Readline_error::CONNECTION_LOST)
            }
        }
        Ok(readen)
    }
}
