extern crate mysql;

use super::Client;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;
use ::random::Random;

use super::main::MainPage;

struct Order {
    client_id: i32,
    text: String
}

pub trait AdminMenu {
    fn admin_menu(&mut self) -> ();
}

impl<'a> AdminMenu for Client<'a>
{
    fn admin_menu(&mut self)
    {

        self.stream.write(b"             ---> Admin panel <---\n\
                            1. Log in\n\
                            2. What the shit, give me the freaking password!\n\
                            3. Back\n").unwrap();

        loop {
            self.stream.write(b"\n> ").unwrap();
            let mut buf = String::new();
            self.stream.read_line(&mut buf).unwrap();

            if buf.len() != 1 {continue}

            match buf.chars().next().unwrap() {
                '1' => {
                    self.stream.write(b"Type secret word: ").unwrap();

                    let mut buf = String::new();
                    self.stream.read_line(&mut buf).unwrap();


                    if { self.config.lock().unwrap().admin_passwd != buf } {       // Так надо.
                        self.stream.write(b"Wrong!\n").unwrap();
                    } else {
                        break;
                    }
                },
                '2' => {
                    let mut new_passwd = String::new();

                    while new_passwd.len() < 6 {
                        let rnd = (self.rand.rand_u32() % 62) as u8;
                        let c = match rnd {
                            n @  0...25 => 0x61 + n,
                            n @ 26...51 => 0x41 + n-26,
                            n @    _    => 0x30 + n-52
                        } as char;
                        new_passwd.push(c);
                    }

                    {
                        self.config.lock().unwrap()
                            .admin_passwd = new_passwd.clone();
                    }

                    println!("Admin password has been reset. New password: {}", new_passwd);
                    self.stream.write(b"Admin password has been reset and has been printed in the console.\n").unwrap();
                },
                '3' => {
                    return self.main_page();
                },
                _ => {continue}
            } // end match
        } // end loop

        self.stream.write(b"Well log in!\n
                            \n\
                            Admin menu:\n\
                            1. Full orders list\n\
                            2. Truncate database\n\
                            3. Back\n").unwrap();

        loop {
            self.stream.write(b"\n> ").unwrap();
            let mut buf = String::new();
            self.stream.read_line(&mut buf).unwrap();

            if buf.len() != 1 {continue}

            match buf.chars().next().unwrap() {
                '1' => {
                    let pool = self.conn.lock().unwrap();
                    let orders: Vec<Order> =
                    pool.prep_exec("SELECT * from `shawerma`.`orders` ORDER BY `shawerma`.`orders`.`client_id` ASC", ())
                    .map(|result| {
                        result.map(|x| x.unwrap()).map(|row| {
                            let (client_id, text) = mysql::from_row(row);
                            Order {
                                client_id: client_id,
                                text: text
                            }
                        }).collect()
                    }).unwrap();

                    for order in orders {
                        self.stream.write( format!("Conn #{}: {}\n", order.client_id, order.text).as_bytes() ).unwrap();
                    }
                },
                '2' => {
                    let pool = self.conn.lock().unwrap();
                    match pool.prep_exec("TRUNCATE `shawerma`.`orders`", ()) {
                        Ok(_)  => { self.stream.write(b"Done!\n").unwrap(); },
                        Err(_) => { self.stream.write(b"Error.\n").unwrap(); }
                    }
                },
                '3' => { return self.main_page(); },
                _ => {}
            }
        }


    }
}
