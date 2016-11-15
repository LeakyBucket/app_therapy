use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::{Nonce, PublicKey, SecretKey};

pub fn new_box(contents: &[u8], pub_key: &PublicKey, priv_key: &SecretKey) -> (box_::Nonce, Vec<u8>) {
    let nonce = box_::gen_nonce();

    (nonce, box_::seal(contents, &nonce, &pub_key, &priv_key))
}

pub fn re_box(contents: &[u8], pub_key: &PublicKey, priv_key: &SecretKey, nonce: &Nonce) -> Vec<u8> {
    box_::seal(contents, &nonce, &pub_key, &priv_key)
}

pub fn un_box(boxed: &[u8], nonce: &Nonce, pub_key: &PublicKey, priv_key: &SecretKey) -> Result<Vec<u8>, ()> {
    box_::open(boxed, nonce, pub_key, priv_key)
}

pub fn to_nonce(value: &[u8]) -> Option<Nonce> {
    Nonce::from_slice(value)
}

pub fn to_pub(key: &[u8]) -> Option<PublicKey> {
    PublicKey::from_slice(key)
}

pub fn to_priv(key: &[u8]) -> Option<SecretKey> {
    SecretKey::from_slice(key)
}

pub fn generate_keys() {
    let (pub_key, priv_key) = box_::gen_keypair();
    let mut pub_file = match File::create(&"./app_therapy.pub") {
        Err(reason) => panic!("Couldn't create public key: {}", reason.description()),
        Ok(file) => file,
    };
    let mut priv_file = match File::create(&"./app_therapy.priv") {
        Err(reason) => panic!("Couldn't create private key: {}", reason.description()),
        Ok(file) => file,
    };

    match pub_file.write_all(&pub_key.0) {
        Err(why) => panic!("Couldn't write public key file: {}", why.description()),
        Ok(_) => println!("Created ./app_therapy.pub"),
    }

    match priv_file.write_all(&priv_key.0) {
        Err(why) => panic!("Couldn't write private key file: {}", why.description()),
        Ok(_) => println!("Created ./app_therapy.priv"),
    }
}
