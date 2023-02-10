
use zettlekasten::{*, rdb::*, in_out::*};

fn main() {
    let mut z = zettlekasten::rdb::new();
    let mut input = String::new();
    // load the database from a file
    z.load("test.txt");
    // add a key-value pair to the database
    z.add("key".to_string(), "value".to_string());
    // delete a key-value pair from the database
    z.delete("key".to_string());
    // update a key-value pair in the database
    z.update("key".to_string(), "value".to_string());
    // query the database for a key
    println!("{:?}", z.query("key".to_string()));
    // save the database to a file
    z.save("test.txt");
}
