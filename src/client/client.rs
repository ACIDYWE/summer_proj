use std::net::TcpStream;

pub struct Client<'a> {pub stream: &'a mut TcpStream}

impl<'a> Client<'a>
{
    pub fn new (s: &'a mut TcpStream) -> Client
    {
        Client{stream: s}
    }
}
