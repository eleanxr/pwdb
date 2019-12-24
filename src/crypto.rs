use openssl::sha::sha256;
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::pkcs5;

// TODO: Generate
pub fn initialization_vector() -> [u8; 16] {
    *b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07"
}

pub fn hash_string(s: &str) -> [u8; 32] {
    sha256(s.as_bytes())
}

fn run_crypter(key: [u8; 16], iv: [u8; 16], mode: Mode, data: &Vec<u8>) -> Result<Vec<u8>, String> {
    let length = data.len();
    let mut encrypter =
        Crypter::new(Cipher::aes_128_cbc(), mode, &key, Some(&iv)).unwrap();
    let block_size = Cipher::aes_128_cbc().block_size();

    let mut output = vec![0; length + block_size];
    let mut count = encrypter.update(&data, &mut output).unwrap();
    count += encrypter.finalize(&mut output[count..]).unwrap();
    output.truncate(count);
    Ok(output)
}

pub fn encrypt_string(key: [u8; 16], iv: [u8; 16], s: &str) -> Result<Vec<u8>, String> {
    run_crypter(key, iv, Mode::Encrypt, &s.as_bytes().to_vec())
}

pub fn decrypt_string(key: [u8; 16], iv: [u8; 16], data: &Vec<u8>) -> Result<String, String> {
    String::from_utf8(run_crypter(key, iv, Mode::Decrypt, &data).unwrap())
        .map_err(|err| err.to_string())
}

pub fn derive_key(password: &String) -> Result<[u8; 16], String> {
    let mut key: [u8; 16] = [0; 16];
    let result = pkcs5::scrypt(
        password.as_bytes(),
        password.as_bytes(),
        16384,
        8,
        1,
        0,
        &mut key,
    );
    match result {
        Err(stack) => Err(stack.to_string()),
        _ => Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_encrypt_string() {
        let key1 = derive_key(&String::from("pw1"));
        let key2 = derive_key(&String::from("pw2"));
        let plaintext = String::from("some plaintext goes here");

        let c1 = encrypt_string(key1.unwrap(), initialization_vector(), &plaintext).unwrap();
        let c2 = encrypt_string(key2.unwrap(), initialization_vector(), &plaintext).unwrap();
        println!("c1: {}", base64::encode(&c1));
        println!("c2: {}", base64::encode(&c2));
        assert!(c1 != c2);
    }
}
