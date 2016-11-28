use std::io::Read;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::net::TcpStream;

pub fn process_request(mut stream: TcpStream) {
pub fn process_request(stream: &mut TcpStream) {
    let length = match stream.take(8).read_u64::<NetworkEndian>() {
        Ok(len) => len,
        Err(_) => {
            let _ = stream.write("Bad Message Size!".as_bytes());
            return();
        }
    };

    let _ = stream.read_to_end(&mut request);

    println!("{:?}", request);
}
