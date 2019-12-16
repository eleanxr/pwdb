use std::collections::BTreeMap;

use crate::crypto;

pub struct MapStore {
    map: BTreeMap<[u8; 32], Vec<u8>>,
}

impl MapStore {
    pub fn new() -> MapStore {
        MapStore { map: BTreeMap::new() }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.map
            .insert(crypto::hash_string(key), value.as_bytes().to_vec());
    }

    pub fn find(&self, key: &str) -> Option<String> {
        self.map
            .get(&crypto::hash_string(key))
            .map(|bytes: &Vec<u8>| -> String { String::from_utf8(bytes.to_vec()).unwrap() })
    }
}
