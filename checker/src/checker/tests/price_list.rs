use ::Checker;
use ::helper_func::*;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineError;

pub trait PriceListTest {
    fn test_price_list(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>;
}

impl PriceListTest for Checker
{
    fn test_price_list(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>
    {
        checker_test_write_try!(stream.write(b"1\n"));
        let res = lines_check(stream,
            &[
                "*--------- Price list ---------*",
                "* 0. Tomato        - 1$        *",
                "* 1. Cucumber      - 0.5$      *",
                "* 2. Pepper        - 0.3$      *",
                "* 3. Onion         - 0.7$      *",
                "* 4. Garlic        - 0.2$      *",
                "* 5. Mayonnaise    - 0.4$      *",
                "* 6. Ketchup       - 0.45$     *",
                "* 7. Salt          - 0.1$      *",
                "* 8. Potato        - 0.8$      *",
                "* 9. Chicken       - 1.5$      *",
                "* A. Pork          - 2$        *",
                "* B. Beef          - 3.5$      *",
                "* C. Rice          - 4$        *",
                "* D. Bugs          - 17$       *",
                "* E. Tabasko       - 5$        *",
                "* F. Pray to satan - for free! *",
                "*------------------------------*",
                ""
            ]
        );

        checker_test_try!(res);
        checker_test_try!(bytes_expect(stream, b"> "));

        Ok(true)
    }
}
