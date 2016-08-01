extern crate time;

use super::{CheckerWatch, WatchResult, CheckerErr};

use ::TIME_ROUND;
use ::ReadlineForTcpStream;
use ::ReadlineError;
use ::helper_func::*;

use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;

pub trait CheckerWatchProcess
{
    // С определённым интервалом запрашивает список заказов
    // по self.stream и сверяет с self.flags. Если какой-либо
    // флаг отсутствует (допускается наличие других флагов)
    // или соединение было разорвано, функция вернёт
    // соответствующий Err(). По прошествии TIME_ROUND секунд,
    // ф-ция завершает работу и возвращает Ok(())
    fn watch(&mut self) -> WatchResult;

    // Функция выполняет тоже самое, что и её self.watch(),
    // но возвращает ответ через std::sync::mpsc::Sender
    fn watch_and_send(&mut self, send: Sender<WatchResult>) -> ();
}

#[inline(always)]
fn get_time() -> i64 {time::now().to_timespec().sec}

impl CheckerWatchProcess for CheckerWatch
{
    fn watch(&mut self) -> WatchResult
    {
        let start_time = get_time();

        loop {
            if get_time() - start_time >= TIME_ROUND-10 { break; }

            // Выбираем 3 пункт меню (Orders history)
            match self.stream.write(b"3\n") {
                Ok(_)  => (),
                Err(_) => return Err(CheckerErr::ServerOffline)
            }

            // Начинаем получать ответ. В первой строке
            // ожидается: "Orders list for connection #N"
            checker_try!(bytes_expect(&mut self.stream, b"Orders list for connection #"), CheckerErr::BadOrderHistory);

            let mut buf = String::new();
            match self.stream.read_line(&mut buf) {
                Ok(_)  => (),
                Err(ReadlineError::ConnectionLost) => return Err(CheckerErr::ServerOffline),
                Err(ReadlineError::NotAscii) => return Err(CheckerErr::UnicodeDetected)
            }

            // Orders list for connection #0
            // ............................^ u64 expect
            match buf.parse::<u64>() {
                Ok(_)  => (),
                Err(_) => return Err(CheckerErr::BadOrderHistory)
            }

            // m[i] показывает, был ли обнаружен flag[i]
            // в выводе
            let mut m = vec![false; self.flags.len()];

            // Начинаем считывать историю заказов
            loop {

                // Считываем первые 2 байта. Если это "> ",
                // значит вывод заказов окончен.
                let mut buf = [0u8; 3];
                match self.stream.read_exact(&mut buf) {
                    Ok(_)  => (),
                    Err(_) => return Err(CheckerErr::ServerOffline)
                };

                if &buf == b"\n> " {break}

                // Окей, это не те два байта. Тогда считаем
                // строку до конца и объеденим.

                let mut buf = String::from_utf8(buf.to_vec()).unwrap();

                let mut buf2 = String::new();
                match self.stream.read_line(&mut buf2) {
                    Ok(_)  => (),
                    Err(ReadlineError::ConnectionLost) => return Err(CheckerErr::ServerOffline),
                    Err(ReadlineError::NotAscii) => return Err(CheckerErr::UnicodeDetected)
                }

                buf.push_str(buf2.as_str());

                let _ = buf2; // Пока-пока!

                // Order: 01234567...
                // .......^ 7 symb.
                let (a, b) = buf.split_at(7);
                if a != "Order: " { return Err(CheckerErr::BadOrderHistory); }
                let buf = String::from(b);

                // Сверяем текущий заказ со флагами
                for (i,flag) in (&self.flags).iter().enumerate() {
                    if *flag == buf {
                        m[i] = true; // Отмечаем, что этот флаг присутствует в выводе
                        break;
                    }
                }
            }

            for x in m {
                // Если какой-либо из флагов отсутствует, return Err(..)
                if x == false { return Err(CheckerErr::FlagLost) }
            }

            thread::sleep(Duration::new(5, 0)); // sleep for 5 sec 0 nanosec
        }

        Ok(())
    }

    fn watch_and_send(&mut self, tx: Sender<WatchResult>)
    {
        match tx.send(self.watch()) {
            Ok(_)  => (), // yeah
            Err(_) => ()  // Похоже, reciver уже был уничтожен
        };
    }
}
