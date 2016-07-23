extern crate summer_proj;

use std::io::{Stdin, Write};
use std::net::{TcpListener, TcpStream};
use summer_proj::ReadlineForTcpStream;
use std::thread;
use std::net::SocketAddr;
use std::io::Read;
use std::vec::Vec;
use std::fmt;

//use::std::time::Duration
//is for sleeps

//use summer_proj::ReadlineForTcpStream;

fn main() {
    let SERVER = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("SERVER STARTED!");
    for stream in SERVER.incoming() {
        thread::spawn( || {
            let mut stream = stream.unwrap();
            let mut buffer = String::new();
            stream.write(b"Hello pidr!\n");
            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin
            println!("Some user wrote this: {}\n\
                      And it took - {} bytes", buffer.clone(), stream.read_line(&mut buffer));
            //stream.read_to_end(&mut buffer).unwrap();
            //println!(format!("{}", buffer.as_slice()));
            //stream.write(format!("YOU WROTE: {}", buffer));
        });
    }
}
