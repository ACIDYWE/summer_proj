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
    if n > 60 {
        if n%30 != 0 {
            thread::sleep(Duration::new(n%30, 0));
            n -= n%30;
        }
        print!("{}", n); flush();
        _timer_fn(n, 60, 30, 2);
        _timer_fn(60, 30, 15, 2);
        _timer_fn(30, 5, 5, 2);
        _timer_fn(5, 3, 1, 2);
        _timer_fn(3, 0, 1, 4);
    } else if n > 30 {
        if n%15 != 0 {
            thread::sleep(Duration::new(n%15, 0));
            n -= n%15;
        }
        print!("{}", n); flush();
        _timer_fn(n, 30, 15, 2);
        _timer_fn(30, 5, 5, 2);
        _timer_fn(5, 0, 1, 4);
    } else if n > 5 {
        if n%5 != 0 {
            thread::sleep(Duration::new(n%5, 0));
            n -= n%5;
        }
        print!("{}", n); flush();
        _timer_fn(n, 5, 5, 2);
        _timer_fn(5, 0, 1, 4);
    } else {
        print!("{}", n); flush();
        _timer_fn(n, 0, 1, 4);
    }
    println!("");
}

fn _timer_fn(s: u64, e: u64, step: u64, k: u64)
{
    if s == e { return; }
    assert!(s > e && (s-e)%step == 0 && k > 1);

    let mut t = k;
    while t&1 == 0 { t >>= 1; }
    while t%5 == 0 { t /= 5; }
    assert!(t == 1);

    //let step = (s-e)/n;
    let (secs, nanos) = (step/k, (1000000000 * (step%k)) / k);
    let sl_time = Duration::new(secs, nanos as u32);

    for sec in (e..s).filter(|x| (x-e)%step==0).rev() {
        for _ in 0..(k-1) {
            thread::sleep(sl_time);
            print!("."); flush();
        }
        thread::sleep(sl_time);
        print!("{}", sec); flush();
    }
}

#[inline(always)]
fn flush() {
    io::stdout().flush().unwrap();
}
