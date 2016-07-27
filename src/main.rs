extern crate summer_proj;
extern crate mysql;

use std::thread;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use summer_proj::client::*;
use summer_proj::random::Random;

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
    let mut next_client_id = 0u8;

    for stream in server.incoming() {
        let pool = pool.clone();
        let next_client_id = {let t = next_client_id; next_client_id = next_client_id.wrapping_add(1); t};
        thread::spawn(move || {
            let mut rand = Random::new(12u32);
            let mut stream = stream.unwrap();

            println!("Got connection from: {}", stream.peer_addr().unwrap()); //for Admin

            let mut client = Client{stream: &mut stream, conn: pool, client_id: next_client_id, rand: rand};
            client.main_page();
        });
    }
}
