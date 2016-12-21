use byteorder::{NetworkEndian, ReadBytesExt};
use crypto;
use crypto::FileBacked;
use messaging::{Message, Payload};
use sodiumoxide::crypto::box_::{PublicKey, SecretKey};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

pub fn process_request(stream: &mut TcpStream, agent_pub: PublicKey, key_list: &Arc<HashMap<String, SecretKey>>) {
    let length = match stream.take(8).read_u64::<NetworkEndian>() {
        Ok(len) => len,
        Err(_) => {
            let _ = stream.write("Bad Message Size!".as_bytes());
            return();
        }
    };

    // Create buffer to hold  message and fill it
    let mut message = vec![0; length as usize];
    let _ = stream.take(length).read_to_end(&mut message);

    // Deconstruct the message
    let parts = match Payload::new(&message) {
        Some(payload) => payload,
        None => {
            let _ = stream.write("Failed to process the client message".as_bytes());
            return
        }
    };

    println!("Parts: {:?}", &parts);

    // Find secret key for user that sent message
    let user_key = match key_list.get(&parts.requestor) {
        Some(key) => key.clone(),
        None => {
            let mut message = "No key for".to_string();
            message.push_str(&parts.requestor);

            let _ = stream.write(message.as_bytes());
            return
        }
    };

    println!("User key: {:?}", &user_key);

    // Decrypt the box
    let command = match crypto::un_box(&parts.the_box, &parts.nonce, &agent_pub, &user_key) {
        Ok(plaintext) => plaintext,
        Err(error) => {
            println!("{:?}", error);
            let _ = stream.write("Error: Couldn't decrypt message".as_bytes());
            println!("Decryption error: {:?}", error);
            return
        }
    };

    println!("Instruction {:?}", &command);
}

//fn process_box(payload: &Payload) -> Result<Vec<u8>, Error> {
//
//}

pub fn load_keys(keys: &Vec<Vec<String>>) -> HashMap<String, SecretKey> {
    let mut key_map: HashMap<String, SecretKey> = HashMap::new();

    for key_set in keys {
        let key = match SecretKey::read_from(&key_set[1]) {
            Some(secret_key) => secret_key,
            None => continue
        };

        key_map.insert(key_set[0].clone(), key);
    }

    key_map
}
