use std::net::TcpStream;
pub trait Page {
    fn process(&self, _: &mut TcpStream);
}
