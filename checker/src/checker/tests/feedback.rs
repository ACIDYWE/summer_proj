use ::Checker;
use ::helper_func::*;

use std::net::TcpStream;
use std::io::Write;
use ::ReadlineError;

pub trait FeedbackTest {
    fn test_feedback(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>;
}

impl FeedbackTest for Checker
{
    fn test_feedback(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>
    {
        checker_test_write_try!(stream.write(b"5\n\n"));
        let res = lines_check(stream,
            &[
                "Here you can left a little comment about our service",
                "Heeeey come on bro, write something!",
                ""
            ]
        );

        checker_test_try!(res);
        checker_test_try!(bytes_expect(stream, b"> "));

        Ok(true)
    }
}
