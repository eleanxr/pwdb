
mod crypto;
mod json_store;
mod store;

// TODO Derive
fn key() -> [u8; 16] {
    *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"
}

fn test<T: store::DataStore<String, String>>(data_store: &mut T) {
    let key1: String = String::from("somesite.com");
    let value1: String = String::from("somepassword");
    let key2: String = String::from("anothersite.com");
    let value2: String = String::from("anotherpassword");

    data_store.add(&key1, &value1);
    data_store.add(&key2, &value2);

    println!(
        "anothersite.com: {}",
        data_store
            .find(&key1)
            .expect("Failed to find site")
    );
    println!(
        "somesite.com: {}",
        data_store
            .find(&key2)
            .expect("Failed to find site")
    );
}

fn main() {
    let mut data_store: store::MapStore = store::MapStore::create(key());
    test(&mut data_store);
}
