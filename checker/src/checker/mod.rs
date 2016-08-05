extern crate rand;

mod checker;
mod watch;
mod tests;

pub use self::checker::*;
pub use self::watch::*;
pub use self::tests::*;

use std::net::TcpStream;

pub struct Checker {pub addr: String}
pub struct CheckerWatch {pub stream: TcpStream, pub flags: Vec<String>}

pub type CheckerResult = Result<CheckerWatch, CheckerErr>;
pub type WatchResult = Result<(), CheckerErr>;

#[derive(Debug)]
pub enum CheckerErr {
    BadMainMenu,
    BadAdminLogIn,
    BadPriceList,
    BadGetOrder,
    BadOrderHistory,
    BadCheckYourLuck,
    BadFeedback,
    ServerOffline,
    UnicodeDetected,
    FlagLost,
    TimeOut
    // etc.
}
