// module to handle keywords
// Usage: zettlekasten::keywords::tags(:String) -> Vec<String>);

// ToDos:
// 1. create a fucntion that alows the user to add a new stop word to the stop word list


// function to break a string into a vector of keywords
fn break_string(string: String) -> Vec<String> {
    let mut keywords: Vec<String> = Vec::new();
    let mut word: String = String::new();
    for c in string.chars() {
        if c == ' ' {
            keywords.push(word);
            word = String::new();
            }
        else {
            word.push(c);
            }
        }
    keywords.push(word);
    keywords
    }

// function to remove punctuation from a vector of keywords
fn remove_punctuation(keywords: Vec<String>) -> Vec<String> {
    let mut new_keywords: Vec<String> = Vec::new();
    for word in keywords.iter() {
        let mut new_word: String = String::new();
        for c in word.chars() {
            if c != '.' && c != ',' && c != ';' && c != ':' && c != '!' && c != '?' && c != '(' && c != ')' && c != '[' && c != ']' && c != '{' && c != '}' && c != '"' && c != '\'' && c != '-' && c != '_' && c != '/' && c != '\\' && c != '|' && c != '&' && c != '*' && c != '#' && c != '%' && c != '$' && c != '@' && c != '^' && c != '~' && c != '`' && c != '=' && c != '+' && c != '<' && c != '>' && c != '0' && c != '1' && c != '2' && c != '3' && c != '4' && c != '5' && c != '6' && c != '7' && c != '8' && c != '9' {
                new_word.push(c);
                }
            }
        new_keywords.push(new_word);
        }
    new_keywords
    }

// function to remove stop words from a vector of keywords
fn remove_stop_words(keywords: Vec<String>) -> Vec<String> {
    let mut new_keywords: Vec<String> = Vec::new();
    for word in keywords.iter() {
        if word != "a" && word != "an" && word != "the" && word != "and" && word != "but" && word != "or" && word != "for" && word != "nor" && word != "so" && word != "yet" && word != "as" && word != "at" && word != "by" && word != "for" && word != "from" && word != "in" && word != "into" && word != "of" && word != "on" && word != "to" && word != "with" && word != "without" && word != "about" && word != "above" && word != "across" && word != "after" && word != "against" && word != "along" && word != "amid" && word != "among" && word != "around" && word != "before" && word != "behind" && word != "below" && word != "beneath" && word != "beside" && word != "besides" && word != "between" && word != "beyond" && word != "but" && word != "concerning" && word != "despite" && word != "down" && word != "during" && word != "except" && word != "excepting" && word != "excluding" && word != "following" && word != "inside" && word != "like" && word != "minus" && word != "near" && word != "next" && word != "of" && word != "off" && word != "on" && word != "onto" && word != "opposite" && word != "outside" && word != "over" && word != "past" && word != "per" && word != "plus" && word != "regarding" && word != "round" && word != "save" && word != "since" && word != "than" && word != "through" && word != "to" && word != "toward" && word != "towards" && word != "under" && word != "underneath"
        && word != "unlike" && word != "until" && word != "up" && word != "upon" && word != "versus" && word != "via" && word != "with" && word != "within" && word != "without" {    
            new_keywords.push(word.to_string().to_ascii_lowercase());
            }
        }
    new_keywords
    }       

// function to remove duplicates from a vector of keywords
fn remove_duplicates(keywords: Vec<String>) -> Vec<String> {
    let mut new_keywords: Vec<String> = Vec::new();
    for word in keywords.iter() {
        if !new_keywords.contains(word) {
            new_keywords.push(word.to_string());
            }
        }
    new_keywords
    }

// function to remove keywords that are too short
fn remove_short_keywords(keywords: Vec<String>) -> Vec<String> {
    let mut new_keywords: Vec<String> = Vec::new();
    for word in keywords.iter() {
        if word.len() > 4 {
            new_keywords.push(word.to_string());
            }
        }
    new_keywords
    }

// public function to break a string into a vector of keywords
pub fn tags(string: String) -> Vec<String> {
    let keywords: Vec<String> = break_string(string);
    let keywords: Vec<String> = remove_punctuation(keywords);
    let keywords: Vec<String> = remove_stop_words(keywords);
    let keywords: Vec<String> = remove_duplicates(keywords);
    let keywords: Vec<String> = remove_short_keywords(keywords);
    keywords
    }

// function to alow the user to add a new stop word to the stop word list
//fn add_stop_word(stop_word: String) {
//    let mut stop_words: Vec<String> = Vec::new();
//    stop_words.push(stop_word);
//    }