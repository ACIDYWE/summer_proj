extern crate mysql;

use super::Client;

use std::io::Write;
use std::net::TcpStream;
use ::ReadlineForTcpStream;

pub trait FeedBack {
	fn feedback(&mut self) -> ();
}

struct Comment {
	id: i32,
	text: String,
}

impl<'a> FeedBack for Client<'a> {
	fn feedback (&mut self) -> () {
		self.stream.write(b"Here you can left a little comment about our service\n").unwrap();
		let mut buf = String::new();
		let len = self.stream.read_line(&mut buf).unwrap();
		if len == 0 { 
			self.stream.write(b"Heeeey come on bro, write something!\n").unwrap();
			return;
		}
		let c_comment = Comment {id: self.client_id as i32,
									text: buf };
		let query = format!("insert into shawerma.feedback(client_id, comment) 
								values ({}, '{}')", c_comment.id, c_comment.text);
		let pool = self.conn.lock().unwrap();
		pool.prep_exec(query, ()).unwrap();
		self.stream.write(b"Thank you!").unwrap();
	}
}

