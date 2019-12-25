mod console;
mod crypto;
mod store;

use std::env;

fn add_entry<T: store::DataStore<String, String>>(data_store: &mut T, site: &String) {
    data_store.add(&String::from("fakepassword"), &site, &String::from("testpassword"));
}

fn get_entry<T: store::DataStore<String, String>>(data_store: &T, site: &String) {
}

fn main() {
    let mut data_store: store::MapStore = store::MapStore::create();
    let command = console::parse_command_line(&env::args().collect()).expect("Invalid invocation");
    match command.operation {
        console::Operation::Add => add_entry(&mut data_store, &command.site),
        console::Operation::Get => get_entry(&data_store, &command.site),
    }
    let json = serde_json::to_string(&data_store).unwrap();
    println!("data: {}", json);
}
