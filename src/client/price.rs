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
        self.stream.write(b"*-------- Price list --------*\n\
                            * 0. Tomato - 1$             *\n\
                            * 1. Cucumber - 0.5$         *\n\
                            * 2. Pepper - 0.3$           *\n\
                            * 3. Onion - 0.7$            *\n\
                            * 4. Garlic - 0.2$           *\n\
							* 5. Mayonnaise - 0.4$       *\n\
							* 6. Ketchup - 0.45$         *\n\
							* 7. Salt - 0.1$             *\n\
							* 8. Potato - 0.8$           *\n\
							* 9. Chicken - 1.5$          *\n\
							* A. Pork - 2$               *\n\
							* B. Beef - 3.5$             *\n\
							* C. Rice - 4$               *\n\
							* D. Bugs - 17$              *\n\
							* E. Tabasko - 5$            *\n\
							* F. Pray to satan for free! *\n\
                            --------------------------\n").unwrap();
    }
}
