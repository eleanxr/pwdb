use openssl::pkcs5;
use openssl::rand;
use openssl::sha::sha256;
use openssl::symm::{Cipher, Crypter, Mode};

pub type SymmetricKey = [u8; 16];
pub type InitializationVector = [u8; 16];
pub type Salt = [u8; 16];

pub fn initialization_vector() -> InitializationVector {
    let mut result: InitializationVector = [0; 16];
    rand::rand_bytes(&mut result).expect("Failed to generate initialization vector.");
    result
}

pub fn salt() -> Salt {
    let mut result: Salt = [0; 16];
    rand::rand_bytes(&mut result).expect("Failed to generate salt.");
    result
}

pub fn hash_string(s: &str) -> [u8; 32] {
    sha256(s.as_bytes())
}

fn run_crypter(
    key: SymmetricKey,
    iv: InitializationVector,
    mode: Mode,
    data: &Vec<u8>,
) -> Result<Vec<u8>, String> {
    let length = data.len();
    let mut encrypter = Crypter::new(Cipher::aes_128_cbc(), mode, &key, Some(&iv))
        .expect("Failed to create encrypted stream.");
    let block_size = Cipher::aes_128_cbc().block_size();

    let mut output = vec![0; length + block_size];
    let total_count = encrypter.update(&data, &mut output).and_then(|count| {
        encrypter
            .finalize(&mut output[count..])
            .and_then(|c2| Ok(count + c2))
    });
    match total_count {
        Ok(_) => Ok(output),
        Err(_) => Err("Encryption/decryption error".to_string()),
    }
}

pub fn encrypt_string(
    key: SymmetricKey,
    iv: InitializationVector,
    s: &str,
) -> Result<Vec<u8>, String> {
    run_crypter(key, iv, Mode::Encrypt, &s.as_bytes().to_vec())
}

pub fn decrypt_string(
    key: SymmetricKey,
    iv: InitializationVector,
    data: &Vec<u8>,
) -> Result<String, String> {
    run_crypter(key, iv, Mode::Decrypt, &data)
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
        .map_err(|e| e.to_string())
}

pub fn derive_key(password: &String, salt: &Salt) -> Result<SymmetricKey, String> {
    let mut key: SymmetricKey = [0; 16];
    let result = pkcs5::scrypt(password.as_bytes(), salt, 16384, 8, 1, 0, &mut key);
    match result {
        Err(stack) => Err(stack.to_string()),
        _ => Ok(key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_encrypt_string() {
        let salt: Salt = [
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
        ];
        let key1 = derive_key(&String::from("pw1"), &salt);
        let key2 = derive_key(&String::from("pw2"), &salt);
        let plaintext = String::from("some plaintext goes here");

        let c1 = encrypt_string(key1.unwrap(), initialization_vector(), &plaintext).unwrap();
        let c2 = encrypt_string(key2.unwrap(), initialization_vector(), &plaintext).unwrap();
        println!("c1: {}", base64::encode(&c1));
        println!("c2: {}", base64::encode(&c2));
        assert!(c1 != c2);
    }
}
