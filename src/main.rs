extern crate summer_proj;
extern crate mysql;

use std::thread;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use summer_proj::client::*;

fn main() {
    let pool = mysql::Pool::new("mysql://root:123456@localhost:3306").unwrap();
    let pool = Arc::new( Mutex::new(pool) );
    let server = TcpListener::bind("127.0.0.1:31337").unwrap();
    println!("    SERVER STARTED!");
    println!("**********************\n\
              *                    *\n\
              *     ADMIN PANEL    *\n\
              *                    *\n\
              **********************\n");
    let mut client_counter = 0u8;

    for stream in server.incoming() {
        let pool = pool.clone();
        let client_counter = match client_counter.checked_add(1) {
            Some(_) => {let t = client_counter; client_counter += 1; t},
            None    => {let t = client_counter; client_counter  = 0; t}
        };

        thread::spawn(move || {
            let mut stream = stream.unwrap();

            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin

            let mut client = Client{stream: &mut stream, conn: pool, client_id: client_counter};
            client.main_page();
        });
    }
}
