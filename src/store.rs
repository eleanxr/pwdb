use std::collections::BTreeMap;

use crate::crypto;

use serde::{Deserialize, Serialize};

pub type SymmetricKey = [u8; 16];
pub type InitializationVector = [u8; 16];
pub type Salt = [u8; 16];
type Hash = [u8; 32];

#[derive(Serialize, Deserialize)]
struct DataBlock {
    salt: Salt,
    initialization_vector: InitializationVector,
    data: Vec<u8>,
}

pub trait DataStore<K, V> {
    fn add(&mut self, symmetric_key: SymmetricKey, key: &K, value: &V);
    fn find(&self, symmetric_key: SymmetricKey, key: &K) -> Result<String, String>;
}

#[derive(Serialize, Deserialize)]
pub struct MapStore {
    map: BTreeMap<String, DataBlock>,
}

impl MapStore {
    pub fn create() -> MapStore {
        MapStore {
            map: BTreeMap::new(),
        }
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}

impl Hashable for String {
    fn hash(&self) -> Hash {
        crypto::hash_string(self.as_str())
    }
}

pub trait Cryptable {
    fn encrypt(&self, symmetric_key: SymmetricKey, iv: InitializationVector) -> Vec<u8>;
    fn decrypt(symmetric_key: SymmetricKey, iv: InitializationVector, data: &Vec<u8>) -> Self;
}

impl Cryptable for String {
    fn encrypt(&self, symmetric_key: SymmetricKey, iv: InitializationVector) -> Vec<u8> {
        crypto::encrypt_string(symmetric_key, iv, self.as_str()).unwrap()
    }

    fn decrypt(symmetric_key: SymmetricKey, iv: InitializationVector, data: &Vec<u8>) -> Self {
        crypto::decrypt_string(symmetric_key, iv, &data).unwrap()
    }
}

impl<K: Hashable, V: Cryptable> DataStore<K, V> for MapStore {
    fn add(&mut self, symmetric_key: SymmetricKey, key: &K, value: &V) {
        let iv = crypto::initialization_vector();
        let mut salt: Salt = [0; 16];
        openssl::rand::rand_bytes(&mut salt).expect("Failed to generate salt.");
        self.map.insert(
            hex::encode(key.hash()),
            DataBlock {
                salt: salt,
                initialization_vector: iv,
                data: value.encrypt(symmetric_key, iv),
            },
        );
    }

    fn find(&self, symmetric_key: SymmetricKey, key: &K) -> Result<String, String> {
        let result = self
            .map
            .get(&hex::encode(key.hash()))
            .map(|block: &DataBlock| {
                Cryptable::decrypt(symmetric_key, block.initialization_vector, &block.data)
            });
        match result {
            Some(value) => Ok(value),
            None => Err(String::from("Failed to decrypt.")),
        }
    }
}
