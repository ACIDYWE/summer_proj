extern crate summer_proj;

use std::io::{Stdin, Write};
use std::net::{TcpListener, TcpStream};
use summer_proj::ReadlineForTcpStream;
use std::thread;
use std::net::SocketAddr;
use std::io::Read;
use std::vec::Vec;
use std::fmt;

use summer_proj::page::*;

//use::std::time::Duration
//is for sleeps

//use summer_proj::ReadlineForTcpStream;

fn main() {
    let SERVER = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("SERVER STARTED!");

    for stream in SERVER.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();

            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin

            let main_page = &summer_proj::pages::MainPage;
            let main_page = Page::new(main_page);

            main_page.process(&mut stream);
        });
    }
}
