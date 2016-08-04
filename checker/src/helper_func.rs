use std::net::TcpStream;
use std::io::Read;

use ::ReadlineForTcpStream;
use ::ReadlineError;

// Обрабатывает значение первого аргумента. Если оно равено
// Ok(false), то макрос делает return [второй аргумент]. Если оно
// равено Err(_), то — return [соотв. ошибка из CheckerErr]
#[macro_export]
macro_rules! checker_try {
    ( $x:expr, $a:expr ) => {
        {
            use ::ReadlineError;
            use ::checker::CheckerErr;
            match $x {
                Ok(b) => if !b { return Err($a); },
                Err(ReadlineError::NotAscii) => return Err(CheckerErr::UnicodeDetected),
                Err(ReadlineError::ConnectionLost) => return Err(CheckerErr::ServerOffline)
            };
        }
    };
}

// Обрабатывает значение, переданное первым аргументом. Если
// оно равно Ok(false), макрос делает return Ok(false). Если
// оно принимает значение Err(e), макрос сделает return Err(e).
#[macro_export]
macro_rules! checker_test_try {
    ( $x:expr ) => {
        {
            match $x {
                Ok(b) => if !b { return Ok(false); },
                Err(e) => return Err(e),
            };
        }
    };
}

// Принимает один аргумент. Если это std::io::Error, делает
// return Err(ReadlineError::ConnectionLost)
#[macro_export]
macro_rules! checker_test_write_try {
    ( $x:expr ) => {
        {
            use ::ReadlineError;
            match $x {
                Ok(_) => (),
                Err(_) => return Err(ReadlineError::ConnectionLost),
            };
        }
    };
}

// Получает из stream строку и если она совпадает с line,
// возвращает Ok(true), в ином случае - Ok(false). Если
// при обращении к stream.read_line(..) возникла ошибка,
// вернёт Err(())
pub fn line_check(stream: &mut TcpStream, line: &str) -> Result<bool, ReadlineError>
{
    let mut buf = String::new();
    try!(stream.read_line(&mut buf));
    Ok(buf.as_str() == line)
}

// Вызывает последовательно с каждой строкой из lines функцию
// line_check(stream, line), если очередной вызов вернул Ok(false),
// ф-ция вернёт Ok(false), если очередной вызов вернул Err(e),
// ф-ция вернёт Err(e). Т.о. ф-ция вернёт Ok(true) если каждый вызов
// line_check(stream, line) вернёт Ok(true)
pub fn lines_check(stream: &mut TcpStream, lines: &[&str]) -> Result<bool, ReadlineError>
{
    for line in lines {
        match line_check(stream, line) {
            Ok(b)  => if !b { return Ok(false) },
            Err(e) => return Err(e)
        };
    };
    Ok(true)
}

// Читает из stream s.len() байт и, если они совпадают с s,
// вернёт Ok(true), в противном случае вернёт Ok(false). Если
// при обращении к stream.read_line(..) возникла ошибка,
// вернёт Err(())
pub fn bytes_expect(stream: &mut TcpStream, s: &[u8]) -> Result<bool, ReadlineError>
{
    let mut buf = vec![0u8; s.len()].into_boxed_slice();

    match stream.read_exact(&mut buf) {
        Ok(_)  => (),
        Err(_) => return Err(ReadlineError::ConnectionLost)
    };

    Ok(*buf == *s)

}
