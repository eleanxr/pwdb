use openssl::sha::sha256;
use openssl::symm::{Cipher, Crypter, Mode};

pub fn hash_string(s: &str) -> [u8; 32] {
    sha256(s.as_bytes())
}

