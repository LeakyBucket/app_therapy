use std::net::TcpStream;
use std::error::Error;

pub fn connect(target: &str) -> TcpStream {
    match TcpStream::connect(target) {
        Ok(connection) => connection,
        Err(e) => panic!("{}", e.description()),
    }
}
