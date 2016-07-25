use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub fn price_list(stream: &mut TcpStream)
{
    stream.write(b"*------ Price list ------*\n\
                   * Dough - 1$             *\n\
                   * Cheese - 0.5$          *\n\
                   * Beef - 0.5$            *\n\
                   * Catsup - 0.5$          *\n\
                   * Praising Satan - free! *\n\
                   --------------------------\n").unwrap();
}
