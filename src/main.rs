extern crate summer_proj;

use std::io::{Read, Write};
use summer_proj::ReadlineForTcpStream;
use std::thread;
use std::net::TcpListener;

use summer_proj::page::Page;

//use::std::time::Duration
//is for sleeps

//use summer_proj::ReadlineForTcpStream;

fn main() {
    let server = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("SERVER STARTED!");
    pritln!("**********************\n\
             *                    *\n\
             *     ADMIN PANEL    *\n\
             *                    *\n\
             **********************\n\n");

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();

            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin

            let main_page = &summer_proj::pages::MainPage;
            let main_page = Page::new(main_page);

            main_page.load_for(&mut stream);
        });
    }
}
