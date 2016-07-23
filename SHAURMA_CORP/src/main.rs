use std::io::{Stdin, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::net::SocketAddr;
use std::io::Read;
use std::vec::Vec;

//use::std::time::Duration
//is for sleeps

fn main() {
    let SERVER = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("SERVER STARTED!");
    for stream in SERVER.incoming() {
        thread::spawn( || {
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            stream.write(b"Hello pidr!\n");
            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin
            stream.read_to_end(&mut buffer).unwrap();
            stream.write(buffer.as_slice());            
        });
    }
}
