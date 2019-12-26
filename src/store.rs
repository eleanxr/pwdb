use std::collections::BTreeMap;

use crate::crypto;
use crypto::{InitializationVector, Salt, SymmetricKey};

use serde::{Deserialize, Serialize};

type Hash = [u8; 32];

#[derive(Serialize, Deserialize)]
struct DataBlock {
    salt: Salt,
    initialization_vector: InitializationVector,
    data: Vec<u8>,
}

pub trait DataStore<K, V> {
    fn add(&mut self, passphrase: &String, key: &K, value: &V);
    fn find(&self, passphrase: &String, key: &K) -> Result<V, String>;
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

pub trait Cryptable
where
    Self: std::marker::Sized,
{
    fn encrypt(&self, symmetric_key: SymmetricKey, iv: InitializationVector) -> Vec<u8>;
    fn decrypt(
        symmetric_key: SymmetricKey,
        iv: InitializationVector,
        data: &Vec<u8>,
    ) -> Result<Self, String>;
}

impl Cryptable for String {
    fn encrypt(&self, symmetric_key: SymmetricKey, iv: InitializationVector) -> Vec<u8> {
        crypto::encrypt_string(symmetric_key, iv, self.as_str()).unwrap()
    }

    fn decrypt(
        symmetric_key: SymmetricKey,
        iv: InitializationVector,
        data: &Vec<u8>,
    ) -> Result<Self, String> {
        crypto::decrypt_string(symmetric_key, iv, &data).map_err(|e| e.to_string())
    }
}

impl<K: Hashable, V: Cryptable> DataStore<K, V> for MapStore {
    fn add(&mut self, passphrase: &String, key: &K, value: &V) {
        let iv = crypto::initialization_vector();
        let salt = crypto::salt();
        let symmetric_key = crypto::derive_key(passphrase, &salt).expect("Failed to derive key.");
        self.map.insert(
            hex::encode(key.hash()),
            DataBlock {
                salt: salt,
                initialization_vector: iv,
                data: value.encrypt(symmetric_key, iv),
            },
        );
    }

    fn find(&self, passphrase: &String, key: &K) -> Result<V, String> {
        match self.map.get(&hex::encode(key.hash())) {
            Some(block) => {
                let symmetric_key = crypto::derive_key(passphrase, &block.salt)
                    .expect("Failed to derive symmetric key.");
                Cryptable::decrypt(symmetric_key, block.initialization_vector, &block.data)
            },
            None => Err("Key not found.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let mut data_store: MapStore = MapStore::create();
        data_store.add(
            &"encryption_password".to_string(),
            &"key1".to_string(),
            &"value1".to_string(),
        );

        let result: String = data_store
            .find(&"encryption_password".to_string(), &"key1".to_string())
            .unwrap();
        assert_eq!("value1".to_string(), result);
    }
}
