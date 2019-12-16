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
    let plaintexts: [&[u8]; 2] = [b"data1", b"data2"];
    let data_len = plaintexts.iter().fold(0, |sum, x| sum + x.len());

    let mut encrypter = Crypter::new(
        Cipher::aes_128_cbc(),
        Mode::Encrypt,
        &key(),
        Some(&initialization_vector()),
    )
    .unwrap();

    let blocksize = Cipher::aes_128_cbc().block_size();
    let mut ciphertext = vec![0; data_len + blocksize];
    let mut count = encrypter.update(plaintexts[1], &mut ciphertext).unwrap();
    count += encrypter
        .update(plaintexts[0], &mut ciphertext[count..])
        .unwrap();
    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);
    let encoded_ciphertext = base64::encode(&ciphertext);
    println!("{}", encoded_ciphertext);

    let decoded_ciphertext = base64::decode(&encoded_ciphertext).unwrap();
    let ciphertext_len = decoded_ciphertext.len();
    let ciphertexts = [&decoded_ciphertext[..9], &decoded_ciphertext[9..]];

    let mut decrypter = Crypter::new(
        Cipher::aes_128_cbc(),
        Mode::Decrypt,
        &key(),
        Some(&initialization_vector()),
    )
    .unwrap();

    let mut plaintext = vec![0; ciphertext_len + blocksize];
    let mut count = decrypter.update(ciphertexts[0], &mut plaintext).unwrap();
    count += decrypter
        .update(ciphertexts[1], &mut plaintext[count..])
        .unwrap();
    count += decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count);
    println!(
        "Decrypted: {}",
        std::str::from_utf8(&plaintext[..]).unwrap()
    );

    println!("Begin");

    let mut data_store = store::MapStore::new();

    data_store.add("somesite.com", "somepassword");
    data_store.add("anothersite.com", "anotherpassword");

    println!(
        "anothersite.com: {}",
        data_store
            .find("anothersite.com")
            .expect("Failed to find site")
    );
    println!(
        "somesite.com: {}",
        data_store
            .find("somesite.com")
            .expect("Failed to find site")
    );
}
