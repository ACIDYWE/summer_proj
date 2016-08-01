use ::Checker;
use ::helper_func::*;

use std::net::TcpStream;
use ::ReadlineError;

pub trait MainMenuTest {
    // По self.stream на момент вызова не должно
    // быть считано ни байта с момента открытия
    // соединения.
    fn test_main_menu(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>;
}

impl MainMenuTest for Checker
{
    fn test_main_menu(&mut self, stream: &mut TcpStream) -> Result<bool, ReadlineError>
    {
        let res = lines_check(stream,
            &[
                "Wellcome to SHAWERMA",
                "Our SHAWERMA best in the world (otvechau)",
                "",
                "Our BEST IN THE WORLD menu:",
                "1. Price list",
                "2. Get order",
                "3. Orders history",
                "4. Check your luck",
                "5. Feedback",
                "6. Exit",
                "",
                // "> " - ЭТО НЕ ОТДЕЛЬНАЯ СТРОКА
            ]
        );

        checker_test_try!(res);
        checker_test_try!(bytes_expect(stream, b"> "));

        Ok(true)
    }
}
