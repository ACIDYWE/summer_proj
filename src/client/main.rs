use super::Client;
use super::price::PriceListPage;
use super::orders_list::OrdersListPage;
use super::order_reg::OrderRegPage;
use super::luck::CheckYourLuckPage;
use super::admin::AdminMenu;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub trait MainPage {
    fn main_page(&mut self) -> ();
}

impl<'a> MainPage for Client<'a> {
    fn main_page (&mut self)
    {
        self.stream.write(b"Wellcome to SHAWERMA\n\
                            Our SHAWERMA best in the world (otvechau)\n\n\
                            Our BEST IN THE WORLD menu:\n\
                            1. Price list\n\
                            2. Get order\n\
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
                '0' => { return self.admin_menu(); /* Exit from the main page & go to the admin menu */ },
                '1' => { self.price_list();  },
                '2' => { self.order_reg();   },
                '3' => { self.orders_list(); },
                '4' => { self.check_your_luck(); },
                '5' => {
                    self.stream.write(b"Goodbye!\n").unwrap();
                    panic!("Somebody chosen \"Exit\"!"); 
                },
                _ => {continue}
            };
        };

    }
}
