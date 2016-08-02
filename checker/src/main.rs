extern crate checker;

use std::io;
use std::io::Write;
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;

pub use checker::{Checker, CheckerWatch, CheckerErr, CheckerResult, WatchResult, CheckerProcess, CheckerWatchProcess, TIME_ROUND, TIME_ON_CHECK};

fn main() {

    // Считываем адреса сервисов. Пустая
    // строка — признак конца списка.
    let mut addrs: Vec<String> = Vec::new();
    loop {
        let mut addr = String::new();
        io::stdin().read_line(&mut addr).unwrap();

        let (addr, _) = addr.split_at(addr.len()-1); // stdin().read_line(..) возвращает
        let addr = String::from(addr);               // строку вместе с '\n', лол

        if addr.len() == 0 { break; }
        addrs.push(addr);
    }

    let addrs = addrs; // В дальнейшем не предполагается изменение списка сервисов
    let mut round = 0usize;

    // Начинаем раунд...
    loop {
        round += 1;
        println!("          === Начало {n} раунда ===", n=round);

        print!("\nПровожу проверки чекера: "); flush();

        let mut rx: Vec< Receiver<CheckerResult> > = Vec::new();

        for addr in &addrs {
            let mut checker = Checker{addr: addr.clone()};
            let (t, r) = channel::<CheckerResult>();
            rx.push(r);

            thread::spawn(move || {
                checker.check_and_send(t);
            });
        }

        timer_for(TIME_ON_CHECK as u64);

        let mut watchers: Vec<(usize, CheckerWatch)> = Vec::new();

        for (i, r) in (&rx).iter().enumerate() {
            let result = match r.try_recv() {
                Ok(res) => res,
                Err(_)  => Err(CheckerErr::TimeOut)
            };

            match result {
                Ok(res) => {
                    println!("{}\t— проверки чекера прошёл", addrs[i]);
                    watchers.push((i,res));
                },
                Err(e) => println!("{}\t— {:?}", addrs[i], e)
            };
        }

        println!("\nСлушаем оставшиеся сервисы на протяжении раунда:");

        let mut rx: Vec< (usize, Receiver<WatchResult>) > = Vec::new();

        for watcher in watchers {
            let (t, r) = channel::<WatchResult>();
            rx.push((watcher.0, r));

            let mut watcher = watcher.1;

            thread::spawn(move || {
                watcher.watch_and_send(t);
            });
        }

        timer_for(TIME_ROUND as u64);

        for (i, r) in rx {
            let result = match r.try_recv() {
                Ok(res) => res,
                Err(_)  => Err(CheckerErr::TimeOut)
            };

            match result {
                Ok(_) => (),
                Err(CheckerErr::ServerOffline) => println!("{}\t— was offline", addrs[i]),
                Err(e) => println!("{}\t— {:?}", addrs[i], e)
            }
        }

        println!("\n");
    }
}

fn timer_for(mut n: u64)
{
    if n > 5 {
        if n%5 != 0 {
            thread::sleep(Duration::new(n%5, 0));
            n -= n%5;
        }
        print!("{}", n); flush();
        for sec in (5..n).filter(|x| x%5==0).rev() {
            thread::sleep(Duration::new(2, 500000000));
            print!("."); flush();
            thread::sleep(Duration::new(2, 500000000));
            print!("{}", sec); flush();
        }
        n = 5;
    }
    else { print!("{}", n); }

    for sec in (0..n).rev() {
        thread::sleep(Duration::new(0, 200000000));
        print!("."); flush();
        thread::sleep(Duration::new(0, 200000000));
        print!("."); flush();
        thread::sleep(Duration::new(0, 200000000));
        print!("."); flush();
        thread::sleep(Duration::new(0, 200000000));
        print!("{}", sec); flush();
    }
    print!("\n"); flush();
}

#[inline(always)]
fn flush() {
    io::stdout().flush().unwrap();
}
