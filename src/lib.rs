pub mod page;
pub mod pages;

use std::net::TcpStream;
use std::io::Read;

pub trait ReadlineForTcpStream {
    fn read_line(&mut self, buf: &mut String) -> Option<usize>;
}

impl ReadlineForTcpStream for TcpStream {
    fn read_line(&mut self, buf: &mut String) -> Option<usize> {      //reading until new line
        let mut temp: [u8;1] = [0;1];
        let mut readen = 0;
        self.read_exact(&mut temp).unwrap();
        while temp[0] != 10 {
            readen+=1;
            (*buf).push(temp[0] as char);
            self.read_exact(&mut temp).unwrap();
        }
        Some(readen)
    }
}
