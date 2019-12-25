mod console;
mod crypto;
mod store;

use std::env;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

fn create_or_open_store(path: &Path) -> Result<store::MapStore, String> {
    if path.exists() {
        match File::open(path) {
            Ok(data) => serde_json::from_reader(data).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(store::MapStore::create())
    }
}

fn add_entry<T: store::DataStore<String, String>>(data_store: &mut T, site: &String) {
    data_store.add(
        &String::from("fakepassword"),
        &site,
        &String::from("testpassword"),
    );
}

fn get_entry<T: store::DataStore<String, String>>(data_store: &T, site: &String) {
    match data_store.find(&String::from("fakepassword"), &site) {
        Ok(data) => println!("{}", data),
        Err(e) => println!("{}", e)
    }
}

fn main() {
    let command = console::parse_command_line(&env::args().collect()).expect("Invalid invocation");
    
    let path = Path::new("./test.json");
    let mut data_store: store::MapStore =
        create_or_open_store(&path).expect("Failed to open file.");
    match command.operation {
        console::Operation::Add => add_entry(&mut data_store, &command.site),
        console::Operation::Get => get_entry(&data_store, &command.site),
    }
    let json = serde_json::to_string(&data_store).unwrap();
    let mut file = File::create(&path).expect("Failed to open file for write.");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");
}
