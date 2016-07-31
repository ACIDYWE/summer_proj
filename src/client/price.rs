use super::Client;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub trait PriceListPage {
    fn price_list(&mut self) -> ();
}

impl<'a> PriceListPage for Client<'a> {
    fn price_list(&mut self)
    {
        self.stream.write(b"*------ Price list ------*\n\
                            * Dough - 1$             *\n\
                            * Cheese - 0.5$          *\n\
                            * Beef - 0.5$            *\n\
                            * Catsup - 0.5$          *\n\
                            * Praising Satan - free! *\n\
                            --------------------------\n").unwrap();
    }
}
