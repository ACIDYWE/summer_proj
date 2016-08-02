extern crate mysql;

use super::Client;

use std::io::Write;
use std::net::TcpStream;
use ::ReadlineForTcpStream;

pub trait OrderRegPage {
	fn order_reg(&mut self) -> ();
	}

struct Order {
	client_id: i32,
	text: String
}


impl<'a> OrderRegPage for Client<'a> {
	fn order_reg(&mut self) -> () {
		self.stream.write(b"Enter your order here: ").unwrap();
		let mut buffer = String::new();
		let len = self.stream.read_line(&mut buffer).unwrap();
		let buffer = buffer.to_lowercase();
		if len == 0 {
			self.stream.write(b"NANANANANA choose something dude!\n").unwrap();
			return; //return user to the main page
		}
		for i in buffer.as_bytes() {
			if !(*i >= 48 && *i <= 57 || *i >= 97 && *i <= 102) {
				self.stream.write(b"Incorrect symbols, sry.\nTry again later.\n").unwrap();
				return; //return user to the main page
			}
		}
		let c_order = Order { client_id: self.client_id as i32,
								   text: buffer };

		let pool = self.conn.lock().unwrap();
		pool.prep_exec("INSERT INTO shawerma.orders
							(client_id, text)
						VALUES
							(:client_id, :text)", (c_order.client_id, c_order.text)).unwrap();


	}
}
