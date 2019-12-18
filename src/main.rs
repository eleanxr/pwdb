use base64;
use openssl::sha::sha256;
use openssl::symm::{Cipher, Crypter, Mode};

mod crypto;
mod store;

// TODO: Generate
fn initialization_vector() -> [u8; 16] {
    *b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07"
}

// TODO Derive
fn key() -> [u8; 16] {
    *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"
}

fn main() {
    let mut data_store = store::MapStore::new();

    data_store.add(key(), initialization_vector(), "somesite.com", "somepassword");
    data_store.add(key(), initialization_vector(), "anothersite.com", "anotherpassword");

    println!(
        "anothersite.com: {}",
        data_store
            .find(key(), initialization_vector(), "anothersite.com")
            .expect("Failed to find site")
    );
    println!(
        "somesite.com: {}",
        data_store
            .find(key(), initialization_vector(), "somesite.com")
            .expect("Failed to find site")
    );
}
