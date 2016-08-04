extern crate rand;

use super::*;
use self::rand::Rng;
use std::net::TcpStream;
use std::io::Write;
use std::sync::mpsc::Sender;

use ::helper_func::*;
use ::FLAGS_COUNT;

pub trait CheckerProcess
{
    // ф-ция, объединяющая различный средства тестирования
    // сервисов из других модулей и применяющая их на сервисе
    // self.addr
    fn check(&mut self) -> CheckerResult;

    // функция выполняет тоже самое, что и self.check(),
    // но возвращает ответ через std::sync::mpsc::Sender
    fn check_and_send(&mut self, send: Sender<CheckerResult>);
}

impl CheckerProcess for Checker
{
    fn check(&mut self) -> CheckerResult
    {
        let mut rng = rand::thread_rng();

        let mut stream = match TcpStream::connect(self.addr.as_str()) {
            Ok(s) => s,
            Err(_) => return Err(CheckerErr::ServerOffline)
        };

        /* <Checking> */

        checker_try!(self.test_main_menu(&mut stream), CheckerErr::BadMainMenu);
        //...........<    выражение для обработки   >..[return it if Ok(false)]
        checker_try!(self.test_price_list(&mut stream), CheckerErr::BadPriceList);

        // etc.

        /* </Checking> */

        /* <flag-gen-and-post> */

        let mut flags: Vec<String> = Vec::new();

        for _ in 0..FLAGS_COUNT {
            let mut flag = String::new();

            // Генерируем случайный флаг
            for _ in 0..32 {
                let r = rng.gen::<u8>() % 16;
                let c = match r {
                    n @ 0...9 => 0x30+n,
                    n @   _   => 0x61+n-10
                } as char;
                flag.push(c);
            };

            // Выбираем второй пункт меню (Get order)
            match stream.write(b"2\n") {
                Ok(_)  => (),
                Err(_) => return Err(CheckerErr::ServerOffline)
            };
            // Получаем ответ. Ожидается "Enter your order here: ";
            checker_try!(bytes_expect(&mut stream, b"Enter your order here: "), CheckerErr::BadGetOrder);

            // Вводим флаг
            match stream.write(flag.as_bytes()) {
                Ok(_)  => (),
                Err(_) => return Err(CheckerErr::ServerOffline)
            };
            match stream.write(b"\n") {
                Ok(_)  => (),
                Err(_) => return Err(CheckerErr::ServerOffline)
            };



            // Ожидается "\n> "
            match bytes_expect(&mut stream, b"\n> ") {
                Ok(b)  => if !b { return Err(CheckerErr::BadGetOrder); }, // Или BadMainMenu, лол
                Err(_) => return Err(CheckerErr::ServerOffline)
            };

            // Запоминаем флаг
            flags.push(flag);
        };

        /* </flag-gen-and-post> */

        Ok(CheckerWatch{stream: stream, flags: flags})
    }

    fn check_and_send(&mut self, tx: Sender<CheckerResult>)
    {
        let _ =  tx.send(self.check()); // ignore answer
    }
}
