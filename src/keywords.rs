// module to handle keywords
// Usage: zettlekasten::keywords::tags(:String) -> Vec<String>);

/*
ToDos:
1. create a fucntion that alows the user to add a new stop word to the stop word list
2. create a function that alows the user to remove a stop word from the stop word list
3. create a keyword struct that contains the keyword and the number of times it appears in the zettlekasten
4. create a function that returns a vector of keyword structs sorted by the keyword
6. create a function that allows the user to add a new keyword to the user-keyword list
7. create a function that allows the user to remove a keyword from the user-keyword list
5. create a vector of user defined keywords that are used to create the user-keyword list
*/

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

// function to check if a word is a keyword
fn is_stop_word(word: &String) -> bool {
    let stop_words: Vec<String> = stop_words();
    if stop_words.contains(word) {
        false
        }
    else {
        true
        }
    }

// function to remove stop words from a vector of keywords
fn remove_stop_words(keywords: Vec<String>) -> Vec<String> {
    let mut new_keywords: Vec<String> = Vec::new();
    for word in keywords.iter() {
        if is_stop_word(word){
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

// vector of stop words
pub fn stop_words() -> Vec<String> {
    let mut stop_words: Vec<String> = Vec::new();
    stop_words.push(String::from("a"));
    stop_words.push(String::from("an"));
    stop_words.push(String::from("the"));
    stop_words.push(String::from("and"));
    stop_words.push(String::from("but"));
    stop_words.push(String::from("or"));
    stop_words.push(String::from("for"));
    stop_words.push(String::from("nor"));
    stop_words.push(String::from("so"));
    stop_words.push(String::from("yet"));
    stop_words.push(String::from("as"));
    stop_words.push(String::from("at"));
    stop_words.push(String::from("by"));
    stop_words.push(String::from("for"));
    stop_words.push(String::from("from"));
    stop_words.push(String::from("in"));
    stop_words.push(String::from("into"));
    stop_words.push(String::from("of"));
    stop_words.push(String::from("on"));
    stop_words.push(String::from("to"));
    stop_words.push(String::from("with"));
    stop_words.push(String::from("without"));
    stop_words.push(String::from("about"));
    stop_words.push(String::from("above"));
    stop_words.push(String::from("across"));
    stop_words.push(String::from("after"));
    stop_words.push(String::from("against"));
    stop_words.push(String::from("along"));
    stop_words.push(String::from("among"));
    stop_words.push(String::from("around"));
    stop_words.push(String::from("at"));
    stop_words.push(String::from("before"));
    stop_words.push(String::from("behind"));
    stop_words.push(String::from("below"));
    stop_words.push(String::from("beneath"));
    stop_words.push(String::from("beside"));
    stop_words.push(String::from("besides"));
    stop_words.push(String::from("between"));
    stop_words.push(String::from("beyond"));
    stop_words.push(String::from("but"));
    stop_words.push(String::from("by"));
    stop_words.push(String::from("despite"));
    stop_words.push(String::from("down"));
    stop_words.push(String::from("during"));
    stop_words.push(String::from("except"));
    stop_words.push(String::from("for"));
    stop_words.push(String::from("from"));
    stop_words.push(String::from("in"));
    stop_words.push(String::from("inside"));
    stop_words.push(String::from("into"));
    stop_words.push(String::from("like"));
    stop_words.push(String::from("near"));
    stop_words.push(String::from("of"));
    stop_words.push(String::from("off"));
    stop_words.push(String::from("on"));
    stop_words.push(String::from("onto"));
    stop_words.push(String::from("out"));
    stop_words.push(String::from("outside"));
    stop_words.push(String::from("over"));
    stop_words.push(String::from("past"));
    stop_words.push(String::from("since"));
    stop_words.push(String::from("through"));
    stop_words.push(String::from("throughout"));
    stop_words.push(String::from("till"));
    stop_words.push(String::from("to"));
    stop_words.push(String::from("toward"));
    stop_words.push(String::from("under"));
    stop_words.push(String::from("underneath"));
    stop_words.push(String::from("until"));
    stop_words.push(String::from("up"));
    stop_words.push(String::from("upon"));
    stop_words.push(String::from("with"));
    stop_words.push(String::from("within"));
    stop_words.push(String::from("without"));
    stop_words.push(String::from("i"));
    stop_words.push(String::from("me"));
    stop_words.push(String::from("my"));
    stop_words.push(String::from("myself"));
    stop_words.push(String::from("we"));
    stop_words.push(String::from("our"));
    stop_words.push(String::from("ours"));
    stop_words.push(String::from("ourselves"));
    stop_words.push(String::from("you"));
    stop_words.push(String::from("your"));
    stop_words.push(String::from("yours"));
    stop_words.push(String::from("yourself"));
    stop_words.push(String::from("yourselves"));
    stop_words.push(String::from("he"));
    stop_words.push(String::from("him"));
    stop_words.push(String::from("his"));
    stop_words.push(String::from("himself"));
    stop_words.push(String::from("she"));
    stop_words.push(String::from("her"));
    stop_words.push(String::from("hers"));
    stop_words.push(String::from("herself"));
    stop_words.push(String::from("it"));
    stop_words.push(String::from("its"));
    stop_words.push(String::from("itself"));
    stop_words.push(String::from("they"));
    stop_words.push(String::from("them"));
    stop_words.push(String::from("their"));
    stop_words.push(String::from("theirs"));
    stop_words.push(String::from("themselves"));
    stop_words.push(String::from("what"));
    stop_words.push(String::from("which"));
    stop_words.push(String::from("who"));
    stop_words.push(String::from("whom"));
    stop_words.push(String::from("this"));
    stop_words.push(String::from("that"));
    stop_words.push(String::from("these"));
    stop_words.push(String::from("those"));
    stop_words.push(String::from("am"));
    stop_words.push(String::from("is"));
    stop_words.push(String::from("are"));
    stop_words.push(String::from("was"));
    stop_words.push(String::from("were"));
    stop_words.push(String::from("be"));
    stop_words.push(String::from("been"));
    stop_words.push(String::from("being"));
    stop_words.push(String::from("have"));
    stop_words.push(String::from("has"));
    stop_words.push(String::from("had"));
    stop_words.push(String::from("having"));
    stop_words.push(String::from("do"));
    stop_words.push(String::from("does"));
    stop_words.push(String::from("did"));
    stop_words.push(String::from("doing"));
    stop_words   
}

