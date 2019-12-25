mod console;
mod crypto;
mod store;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

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

fn prompt_for_passphrase() -> String {
    print!("Data store passphrase: ");
    io::stdout().flush().expect("I/O error");
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase).expect("Failed to read passphrase.");
    passphrase
}

fn add_entry<T: store::DataStore<String, String>>(data_store: &mut T, site: &String) {
    let passphrase = prompt_for_passphrase();
    print!("Secret for key {}: ", site);
    io::stdout().flush().expect("I/O error");
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Failed to read secret.");
    data_store.add(
        &passphrase,
        &site,
        &secret,
    );
}

fn get_entry<T: store::DataStore<String, String>>(data_store: &T, site: &String) {
    let passphrase = prompt_for_passphrase();
    match data_store.find(&passphrase, &site) {
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
