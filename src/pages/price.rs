use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub fn price_list(stream: &mut TcpStream)
{
    stream.write(b"*------ Price list ------*\n").unwrap();
    stream.write(b"* Dough - 1$             *\n").unwrap();
    stream.write(b"* Cheese - 0.5$          *\n").unwrap();
    stream.write(b"* Beef - 0.5$            *\n").unwrap();
    stream.write(b"* Catsup - 0.5$          *\n").unwrap();
    stream.write(b"* Praising Satan - free! *\n").unwrap();
    stream.write(b"--------------------------\n").unwrap();
}
