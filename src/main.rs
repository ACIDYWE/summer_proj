extern crate summer_proj;

use std::io::Write;
use summer_proj::ReadlineForTcpStream;
use std::thread;
use std::net::TcpListener;

use summer_proj::client::*;

fn main() {
    let server = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("    SERVER STARTED!");
    println!("**********************\n\
              *                    *\n\
              *     ADMIN PANEL    *\n\
              *                    *\n\
              **********************\n");

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();

            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin

            let mut client = Client::new(&mut stream);
            client.main_page(); 
        });
    }
}
