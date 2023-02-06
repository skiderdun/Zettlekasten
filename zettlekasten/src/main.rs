
use zettlekasten;

fn main() {
    let stringy: String = String::from("Mushroom contains a significant amount of protein (25%–35%), carbohydrate (51.3%–62.5%), dietary fiber (8.0%–10.4%), vitamins, minerals, and the majority of the essential amino acids (arginine, leucine, lysine, and tryptophan)");
    let stringy: Vec<String> = zettlekasten::keywords::tags(stringy);
    println!("{:#?}", stringy);
}
