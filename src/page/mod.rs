use std::net::TcpStream;
use ::ReadlineForTcpStream;

pub struct Page<'a> { process_fn: &'a Fn(&mut TcpStream)->() }

impl<'a> Page<'a>
{
    pub fn new ( f: &'a Fn(&mut TcpStream)->() ) -> Page
    {
        Page{ process_fn: f }
    }

    pub fn process(&self, stream: &mut TcpStream) -> ()
    {
        (*self.process_fn)(stream);
    }
}
