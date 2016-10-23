use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::{Nonce, PublicKey, SecretKey};

pub fn new_box(contents: &[u8], pub_key: &PublicKey, priv_key: &SecretKey) -> Vec<u8>{
    let nonce = box_::gen_nonce();

    box_::seal(contents, &nonce, &pub_key, &priv_key)
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
