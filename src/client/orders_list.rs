extern crate mysql;

use super::Client;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub trait OrdersListPage {
    fn orders_list(&mut self) -> ();
}

struct Order {
    client_id: i32,
    text: String
}

impl<'a> OrdersListPage for Client<'a> {
    fn orders_list(&mut self)
    {
        let pool = self.conn.lock().unwrap();
        self.stream.write( format!("Orders list for connection #{}\n", self.client_id).as_bytes() ).unwrap();

        let orders: Vec<Order> =
        pool.prep_exec("SELECT * from shawerma.orders WHERE client_id = :id", (self.client_id,))
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
            self.stream.write( format!("Order: {}\n", order.text).into_bytes().as_slice() ).unwrap();
        }
    }
}
