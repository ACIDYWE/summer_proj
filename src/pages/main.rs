use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;

pub fn MainPage (stream: &mut TcpStream)
{
    stream.write(b"Hello pidr!\n").unwrap();
    stream.write(b"Wellcome to SHAWERMA\n").unwrap();
    stream.write(b"Our SHAWERMA best in the world (otvechau)\n\n").unwrap();
    stream.write(b"Our BEST IN THE WORLD menu:\n").unwrap();
    stream.write(b"1. Price list\n").unwrap();
    stream.write(b"2. Buy\n").unwrap();
    stream.write(b"3. Check your luck\n").unwrap();
    stream.write(b"4. Exit\n").unwrap();

    loop {
        stream.write(b"\n> ");
        let mut buffer = String::new();
        let len = stream.read_line(&mut buffer).unwrap();
        if len != 1 {continue}
        let c = buffer.chars().next().unwrap();

        match c {
            '1' => stream.write(b"your select \"Price list\", but IDITE HAHUI\n").unwrap(),
            '2' => stream.write(b"your select \"Buy\", but IDITE HAHUI\n").unwrap(),
            '3' => stream.write(b"your select \"Check your luck\", but IDITE HAHUI\n").unwrap(),
            '4' => {
                stream.write(b"your select \"Exit\", then IDITE HAHUI\n").unwrap();
                panic!("Kakoito pidor vyshel"); // he he he, bydlo-style mod true
            },
            _ => stream.write(b"your select smth shit, but IDITE HAHUI\n").unwrap()
        };
    }

}
