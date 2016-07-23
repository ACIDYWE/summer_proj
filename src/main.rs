use std::io::{Stdin, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::net::SocketAddr;
use std::io::Read;
use std::vec::Vec;
use std::fmt;

//use::std::time::Duration
//is for sleeps

trait READLINE_FOR_TCPSTREAM { 
    fn read_line(&mut self, buf: &mut String) -> usize;
}

impl READLINE_FOR_TCPSTREAM for TcpStream {
    fn read_line(&mut self, buf: &mut String) -> usize{       //reading until new line
        let mut temp: [u8;1] = [0;1];
        let mut READEN = 0;
        self.read_exact(&mut temp);
        while (temp[0] != 10) {
            READEN+=1;
            (*buf).push(temp[0] as char);
            self.read_exact(&mut temp);
            //println!("{}", buf);    
        }
        READEN
        //println!("{}", temp[0] as char);
    }
}

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
