use ::Checker;
use ::MainMenuTest;
use ::helper_func::*;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineError;

pub trait AdminMenuTest {
    fn test_admin_menu(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>;
}

impl AdminMenuTest for Checker
{
    fn test_admin_menu(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>
    {
        checker_test_write_try!(stream.write(b"0\n"));
        let res = lines_check(stream,
            &[
                "             ---> Admin panel <---",
                "1. Log in",
                "2. What the shit, give me the freaking password!",
                "3. Back",
                ""
            ]
        );

        checker_test_try!(res);
        checker_test_try!(bytes_expect(stream, b"> "));

        checker_test_write_try!(stream.write(b"3\n"));
        checker_test_try!(self.test_main_menu(stream));

        Ok(true)
    }
}
