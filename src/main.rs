use base64;

use openssl::symm::{Cipher, Crypter, Mode};

fn main() {
    let plaintexts: [&[u8]; 2] = [b"data1", b"data2"];
    let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let data_len = plaintexts.iter().fold(0, |sum, x| sum + x.len());

    let mut encrypter = Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, key, Some(iv)).unwrap();

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

    let decoded_ciphertext = base64::decode(&encoded_ciphertext);
}
