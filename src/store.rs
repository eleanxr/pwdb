use std::collections::BTreeMap;

use crate::crypto;

type SymmetricKey = [u8; 16];
type InitializationVector = [u8; 16];
type Hash = [u8; 32];

pub trait DataStore<K, V> {
    fn add(&mut self, key: &K, value: &V);
    fn find(&self, key: &K) -> Result<String, String>;
}

struct DataBlock {
    initialization_vector: InitializationVector,
    data: Vec<u8>,
}

pub struct MapStore {
    symmetric_key: SymmetricKey,
    map: BTreeMap<Hash, DataBlock>,
}

impl MapStore {
    pub fn create(symmetric_key: SymmetricKey) -> MapStore {
        MapStore {
            symmetric_key: symmetric_key,
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
    fn add(&mut self, key: &K, value: &V) {
        let iv = crypto::initialization_vector();
        self.map.insert(
            key.hash(),
            DataBlock {
                initialization_vector: iv,
                data: value.encrypt(self.symmetric_key, iv),
            },
        );
    }

    fn find(&self, key: &K) -> Result<String, String> {
        let result = self.map
            .get(&key.hash())
            .map(|block: &DataBlock| {
                Cryptable::decrypt(self.symmetric_key, block.initialization_vector, &block.data)
            });
        match result {
            Some(value) => Ok(value),
            None => Err(String::from("Failed to decrypt."))
        }
    }
}
