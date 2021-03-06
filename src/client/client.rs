extern crate mysql;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use ::random::Random;
use ::config::ServiceConfig;

pub struct Client<'a>
{
    pub stream: &'a mut TcpStream,
    pub conn: Arc< Mutex<mysql::Pool> >,
    pub client_id: u8,
    pub rand: Random,
    pub config: Arc< Mutex<ServiceConfig> >
}
