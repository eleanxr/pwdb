use std::collections::BTreeMap;

use crate::crypto;

pub struct MapStore {
    map: BTreeMap<[u8; 32], Vec<u8>>,
}

impl MapStore {
    pub fn new() -> MapStore {
        MapStore {
            map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, encryption_key: [u8; 16], iv: [u8; 16], key: &str, value: &str) {
        self.map.insert(
            crypto::hash_string(key),
            crypto::encrypt_string(encryption_key, iv, value).unwrap(),
        );
    }

    pub fn find(
        &self,
        encryption_key: [u8; 16],
        iv: [u8; 16],
        key: &str,
    ) -> Result<String, String> {
        self.map
            .get(&crypto::hash_string(key))
            .map(|bytes: &Vec<u8>| crypto::decrypt_string(encryption_key, iv, &bytes))
            .unwrap_or(Err("Failed to decrypt".to_string()))
    }
}
