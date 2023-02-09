// relational database manager (RDB)
// For now, it's just a wrapper around a HashMap
// Later will be a wrapper around a SQLite database
// Create hash map then wrap it in an RDB struct
// RDB struct will have methods to add, delete, update, and query the database
// RDB struct will also have methods to save and load the database to a file

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// RDB struct
#[derive(Debug)]
pub struct RDB {
    pub db: HashMap<String, String>,
}

// function to create a new RDB
pub fn new() -> RDB {
    RDB {
        db: HashMap::new(),
    }
}

// RDB methods
impl RDB {

    // add a key-value pair to the database
    pub fn add(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    // delete a key-value pair from the database
    pub fn delete(&mut self, key: String) {
        self.db.remove(&key);
    }

    // update a key-value pair in the database
    pub fn update(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    // query the database for a key
    pub fn query(&self, key: String) -> Option<&String> {
        self.db.get(&key)
    }

    // load the database from a file
    pub fn load(&mut self, filename: &str) {
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("successfully read {}", display),
        }

        let lines: Vec<&str> = s.split("\r").collect();
        for line in lines {
            let pair: Vec<&str> = line.split(" ").collect();
            self.db.insert(pair[0].to_string(), pair[1].to_string());
        }
    }

    // save the database to a file
    pub fn save(&self, filename: &str) {
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for (key, value) in &self.db {
            let line = format!("{} {}\r ", key, value);
            match file.write_all(line.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }
    }}




