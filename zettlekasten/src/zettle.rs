// module for creating and managing zettels
// zettels are the basic unit of the zettelkasten
// they are indevidual notes that are linked together
// there are two types of zettels: liturature, and permanent
// liturature zettels are notes taken from a book or article
// permeant zettels are notes that are created by the user and are not linked to any external source

//ToDos:
// 1. create a function to access the Crossref REST API to get Liturature Zettle citation data from a doi and return a citation vector


pub mod Zettle {
    use url;
    use url::{Url, ParseError};

    // liturature zettle struct
    pub struct LitZettle {
        pub citation: Vec<String>,
        pub doi: url::Url,
        pub quote: String,
        pub tags: Vec<String>,
        }
    
    // function to create a new liturature zettle
    pub fn new_lit_zettle(citation: Vec<String>, doi: url::Url, quote: String, tags: Vec<String>) -> LitZettle {
        LitZettle {
            citation: citation,
            doi: doi,
            quote: quote,
            tags: tags,
            }
        }
    
    // permanent zettle struct
    pub struct PermZettle {
        pub title: String,
        pub content: String,
        pub tags: Vec<String>,
        }

    // function to create a new permanent zettle
    pub fn new_perm_zettle(title: String, content: String, tags: Vec<String>) -> PermZettle {
        PermZettle {
            title: title,
            content: content,
            tags: tags,
            }
        }
    // module for breaking down a string into a vector of keywords
    pub mod Keyword {
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
        pub fn keywords(string: String) -> Vec<String> {
            let keywords: Vec<String> = break_string(string);
            let keywords: Vec<String> = remove_punctuation(keywords);
            let keywords: Vec<String> = remove_stop_words(keywords);
            let keywords: Vec<String> = remove_duplicates(keywords);
            let keywords: Vec<String> = remove_short_keywords(keywords);
            keywords
            }
        }

    }
