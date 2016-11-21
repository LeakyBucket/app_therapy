use std::io::Read;
use std::net::TcpStream;

pub fn process_request(mut stream: TcpStream) {
    let mut request = vec![0; 1000];

    let _ = stream.read_to_end(&mut request);

    println!("{:?}", request);
}
