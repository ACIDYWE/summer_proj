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
        self.stream.write( format!("Orders list for connection #{}\n", self.client_id).into_bytes().as_slice() ).unwrap();

        let orders: Vec<Order> =
        pool.prep_exec("SELECT * from shawerma.orders WHERE client_id = :id", (self.client_id,))
        .map(|result| { // In this closure we sill map `QueryResult` to `Vec<Payment>`
            // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
            // will map each `MyResult` to contained `row` (no proper error handling)
            // and second call to `map` will map each `row` to `Payment`
            result.map(|x| x.unwrap()).map(|row| {
                let (client_id, text) = mysql::from_row(row);
                Order {
                    client_id: client_id,
                    text: text
                }
            }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        }).unwrap(); // Unwrap `Vec<Payment>`

        for order in orders {
            self.stream.write( format!("Order: {}\n", order.text).into_bytes().as_slice() ).unwrap();
        }
    }
}
