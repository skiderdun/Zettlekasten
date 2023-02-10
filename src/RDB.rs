// relational database manager (RDB)
// For now, it's just a wrapper around a HashMap
// Later will be a wrapper around a SQLite database
// Create hash map then wrap it in an RDB struct
// RDB struct will have methods to add, delete, update, and query the database
// RDB struct will also have methods to save and load the database to a file

use std::collections::HashMap;
use crate::in_out::read_lines;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

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
        let lines = read_lines(filename);
        for line in lines {
            if let Ok(ip) = line {
                let mut parts = ip.split_whitespace();
                let key = parts.next().unwrap().to_string();
                let value = parts.next().unwrap().to_string();
                self.add(key, value);
            }
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
            let line = format!("{} {}\r", key, value);
            match file.write_all(line.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }
    }}