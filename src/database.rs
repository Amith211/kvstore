use std::{collections::HashMap, fs::{OpenOptions}, io::Write};
use crate::custom_error::CustomError;

pub trait KeyValueDataStore<T> {
    fn get(&self, key: &String) -> Option<&String>;
    fn get_all(&self) -> HashMap<String, String>;
    fn insert(&mut self, key: String, value: String) -> Option<String>;
    fn save(&self, params: T) -> Result<(), CustomError>;
}

pub struct FileDataStoreParams {
    pub path: String
}

impl Default for FileDataStoreParams {
    fn default() -> Self {
        Self { path: "kv.db".to_string() }
    }
}

pub struct Database {
    map: HashMap<String, String>,
    save_function: fn(database: &Database, params: FileDataStoreParams) -> Result<(), CustomError>
}

impl Database {
    // Read from database
    pub fn new(f: fn(&Database, FileDataStoreParams) -> Result<(), CustomError>) -> Result<Database, CustomError> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db");

        if let Ok(x) = contents {
            for line in x.lines() {
                let mut chunks = line.splitn(2, '\t');
                let key = chunks.next().expect("no key");
                let value = chunks.next().expect("no value");
                map.insert(key.to_owned(), value.to_owned());
            }
        }

        // populate our map
        Ok(Database { 
            map: map,
            save_function: f,
        })
    }
}

impl Default for Database {
    fn default() -> Self { Database { 
        map: HashMap::new(),
        save_function: save_to_file
    }}
}

impl KeyValueDataStore<FileDataStoreParams> for Database {
    fn get(&self, key: &String) -> Option<&String> {
        self.map.get(key)
    }

    fn get_all(&self) -> HashMap<String, String> {
        self.map.to_owned()
    }

    fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    fn save(&self, params: FileDataStoreParams) -> Result<(), CustomError> {
        Ok((self.save_function)(self, params)?)
    }
}

fn save_to_file(database: &Database, params: FileDataStoreParams) -> Result<(), CustomError> {
    std::println!("Saving Database to '{}'...", &params.path);            
    return match write_databse_to_file(database, &params) {
        Ok(x) => {
            std::println!("Database Saved to '{}'!", &params.path);
            Ok(x)  
        },
        Err(error) => Err(error),
    };
}

fn write_databse_to_file(database: &Database, params: &FileDataStoreParams) -> Result<(), CustomError> {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(&params.path)?;
    
    for key_values in database.get_all().iter().enumerate()  {
        let kv = key_values.1;
        let contents: String = format!("{}\t{}\n", (kv.0), kv.1);

        if let Err(error) = file.write_all(contents.as_bytes())
        {
            return Err(CustomError::from(error));
        }
    };

    Ok(())
}