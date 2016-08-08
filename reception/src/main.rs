extern crate mysql;

use std::thread;
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, Mutex};
use std::io::{Write, Read};
use std::ascii::AsciiExt;

pub trait ReadlineForTcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError>;
}

#[derive(Debug)]
pub enum ReadlineError {
    NotAscii,
    ConnectionLost,
}

impl ReadlineForTcpStream for TcpStream {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, ReadlineError> {
        let mut c = [0u8];
        let mut readen = 0;
        loop {
            if let Err(_) = self.read_exact(&mut c) {
                return Err(ReadlineError::ConnectionLost);
            }
            if c[0] == 10 { break; }

            if !c[0].is_ascii() { return Err(ReadlineError::NotAscii); }
            buf.push(c[0] as char);
            readen += 1;
        }
        Ok(readen)
    }
}

fn give_points(pool: &mysql::Pool, service_id: u64, points: i64, reason: &str)
{
    if points >= 0 {
        pool.prep_exec("UPDATE `checker`.`services` SET `points`=`points`+:points WHERE `id` = :service",
                       (mysql::Value::from(points), mysql::Value::from(service_id))).unwrap();
    } else {
        pool.prep_exec("UPDATE `checker`.`services` SET `points`=`points`-:points WHERE `id` = :service",
                       (mysql::Value::from(-points), mysql::Value::from(service_id))).unwrap();
    }

    pool.prep_exec("INSERT INTO `checker`.`log`(`id`, `service_id`, `event`, `points`) VALUES (0,:service,:event,:peny)",
                   (
                       mysql::Value::from(service_id),
                       mysql::Value::from(reason),
                       mysql::Value::from(points)
                   )
                  ).unwrap();
}

struct Flag {
    // id: u64,
    // flag: String,
    owner_id: u64,
    used: i64
}

fn try_reg_flag(stream: &mut TcpStream, pool: &mysql::Pool, service_id: u64, flag: &str)
{
    let flags: Vec<Flag> = {
        pool.prep_exec("SELECT * from `checker`.`flags` WHERE `flag` = :flag", (mysql::Value::from(flag),))
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (_, _, owner_id, used) = mysql::from_row::<(u64, String, u64, i64)>(row);
                Flag{/*id: id, flag: flag, */owner_id: owner_id, used: used}
            }).collect()
        }).unwrap()
    };

    if flags.len() == 0 {
        stream.write(b"Flag not found\n").unwrap();
    } else if flags[0].used == -1 {
        stream.write(b"Flag so old\n").unwrap();
    } else if (flags[0].used as u64) == service_id {
        stream.write(b"You used this flag\n").unwrap();
        give_points(pool, service_id, -10, "ReusedFlag");
    } else if flags[0].used != 0 {
        stream.write(b"Flag has been registered\n").unwrap();
    } else if flags[0].owner_id == service_id {
        stream.write(b"It's your own flag!\n").unwrap();
    } else {
        stream.write(b"Flag correct\n").unwrap();
        give_points(pool, service_id, 50, "ReusedFlag");
        pool.prep_exec("UPDATE `checker`.`flags` SET `used`=:service_id WHERE `flag` = :flag", (mysql::Value::from(service_id),mysql::Value::from(flag))).unwrap();
    }
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:123456@localhost:3306").unwrap();
    let pool = Arc::new(Mutex::new(pool));

    let server = TcpListener::bind("127.0.0.1:27015").unwrap();

    println!("RECEPTION STARTED!");

    for stream in server.incoming() {
        let pool = pool.clone();

        thread::spawn(move || {
            let mut stream = stream.unwrap();
            println!("Got connection from: {}", stream.peer_addr().unwrap());

            stream.write(b"Enter your token: ").unwrap();
            let mut token = String::new();
            stream.read_line(&mut token).unwrap();

            let services: Vec<u64> = {
                pool.lock().unwrap().prep_exec("SELECT `id` from `checker`.`services` WHERE `token` = :token", (mysql::Value::from(token),))
                .map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        mysql::from_row::<u64>(row)
                    }).collect()
                }).unwrap()
            };

            if services.len() == 0 {
                stream.write(b"Wrong token!\n").unwrap();
                panic!("Wrong token");
            } else if services.len() > 1 {
                stream.write(b"Error: token not unique\n").unwrap();
                panic!("Token not unique");
            } else {
                let service_id = services[0];
                loop
                {
                    stream.write(b"> ").unwrap();
                    let mut flag = String::new();
                    stream.read_line(&mut flag).unwrap();
                    try_reg_flag(&mut stream, &(pool.lock().unwrap()), service_id, flag.as_str());
                }

            }
        });
    }
}
