use super::Client;
use super::price::PriceListPage;
use super::orders_list::OrdersListPage;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub trait MainPage {
    fn main_page(&mut self) -> ();
}

impl<'a> MainPage for Client<'a> {
    fn main_page (&mut self)
    {
        self.stream.write(b"Hello pidr!\n\
                       Wellcome to SHAWERMA\n\
                       Our SHAWERMA best in the world (otvechau)\n\n\
                       Our BEST IN THE WORLD menu:\n\
                       1. Price list\n\
                       2. Order\n\
                       3. Orders history\n\
                       4. Check your luck\n\
                       5. Exit\n").unwrap();

        loop {
            self.stream.write(b"\n> ").unwrap();
            let mut buffer = String::new();
            let len = self.stream.read_line(&mut buffer).unwrap();
            if len != 1 {continue}
            let c = buffer.chars().next().unwrap();

            match c {
                '1' => {
                    self.price_list();
                },
                '2' => {self.stream.write(b"You'r selected \"Buy\", but IDITE HAHUI\n").unwrap();},
                '3' => {
                    self.orders_list();
                },
                '4' => {self.stream.write(b"You'r selected \"Check your luck\", but IDITE HAHUI\n").unwrap();},
                '5' => {
                    self.stream.write(b"You'r selected \"Exit\", then IDITE HAHUI\n").unwrap();
                    panic!("Kakoito pidor vyshel"); // he he he, bydlo-style mod true
                },
                _ => {self.stream.write(b"You'r selected smth shit, but IDITE HAHUI\n").unwrap();}
            };
        };

    }
}
