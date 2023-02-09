
use zettlekasten::{self, io::read_line};

fn main() {
    println!("test keywords::tags()");
    let stringy: String = String::from("Mushroom contains a significant amount of protein (25%–35%), carbohydrate (51.3%–62.5%), dietary fiber (8.0%–10.4%), vitamins, minerals, and the majority of the essential amino acids (arginine, leucine, lysine, and tryptophan)");
    let stringy: Vec<String> = zettlekasten::keywords::tags(stringy);
    println!("{:#?}", stringy);

    println!("test get database from file");
    let mut rdb = zettlekasten::rdb::new();
    rdb.load("test.txt");
    println!("{:#?}", rdb);

    println!("add a key-value pair to the database");
    rdb.add(read_line(), read_line());
    println!("{:#?}", rdb);

    println!("save the database to a file");
    rdb.save("test.txt");
    println!("{:#?}", rdb);
}
