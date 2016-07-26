extern crate mysql;

use super::Client;

use std::io::Write;
use std::net::TcpStream;

pub trait OrderRegPage {
	fn order_reg(&mut self) -> ();
	}

struct Order {
	client_id: i32,
	text: String
}

impl<'a> OrderRegPage for Client<'a> {
	fn order_reg(&mut self) -> () {
		self.stream.write(b"SOSI PISUN\n");
	}
}

