use byteorder::{NetworkEndian, ReadBytesExt};
use messaging::Payload;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn process_request(stream: &mut TcpStream) {
    let length = match stream.take(8).read_u64::<NetworkEndian>() {
        Ok(len) => len,
        Err(_) => {
            let _ = stream.write("Bad Message Size!".as_bytes());
            return();
        }
    };


    println!("{:?}", request);
    let mut message = vec![0; length as usize];
    let _ = stream.take(length).read_to_end(&mut message);

    let parts = Payload::new(&message);

    println!("{:?}", parts);
}
