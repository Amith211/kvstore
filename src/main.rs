pub mod database;
pub mod custom_error;

use database::Database;
use crate::database::{KeyValueDataStore, FileDataStoreParams};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next();
    let value = arguments.next();

    let mut database = Database::default();
    //.expect("Creating Database failed");

    let key_value = match (key.clone(), value.clone()) {
            (Some(key), Some(value)) => Some((key, value)),
            _ => None,
        };

    if let Some((key, value)) = key_value {
        println!("The input key is {} and the input value is {}", key, value);
        
        database.insert(key.to_owned(), value.to_owned());
        if let Err(_x) = database.save(FileDataStoreParams::default()) {}

        database.get(&key);
    }
}