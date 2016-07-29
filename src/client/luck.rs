use super::Client;

use std::net::TcpStream;
use std::io::Write;
use std::str::FromStr;
use ::ReadlineForTcpStream;

pub trait CheckYourLuckPage {
    fn check_your_luck(&mut self) -> ();
}

impl<'a> CheckYourLuckPage for Client<'a> {
    fn check_your_luck(&mut self)
    {
        self.stream.write( format!("Try to guess number in range [{min}; {max}]: ", min=u32::min_value(), max=u32::max_value()).as_bytes() ).unwrap();
        let mut buf = String::new();
        self.stream.read_line(&mut buf).unwrap();

        match u32::from_str(buf.as_str()) {
            Ok(n) => {
                let m = self.rand.rand_u32();
                if n == m {
                    self.stream.write(b"You're so lucky!\n").unwrap();
                } else {
                    self.stream.write( format!("Sry, but {} was correct :(\n", m).as_bytes() ).unwrap();
                }
            },
            Err(_) => {self.stream.write(b"TI VTIRAEH MNE KAKU'U-TO DICH\n").unwrap();}
        }
    }
}
