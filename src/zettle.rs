// module for creating and managing zettels
// zettels are the basic unit of the zettelkasten
// they are indevidual notes that are linked together
// there are two types of zettels: liturature, and permanent
// liturature zettels are notes taken from a book or article
// permeant zettels are notes that are created by the user and are not linked to any external source

//ToDos:
// 1. create a function to access the Crossref REST API to get Liturature Zettle citation data from a doi and return a citation vector


pub mod zettle {
    use url;
    use crate::keywords::tags;

    // liturature zettle struct
    pub struct LitZettle {
        pub citation: Vec<String>,
        pub doi: url::Url,
        pub quote: String,
        pub tags: Vec<String>,
        }
    
    // function to create a new liturature zettle
    pub fn new_lit_zettle(citation: Vec<String>, doi: url::Url, quote: String) -> LitZettle {
        let quotes: &String = &quote;
        let tags: Vec<String> = tags(quotes.to_string());
        LitZettle {
            citation: citation,
            doi: doi,
            quote: quotes.to_string(),
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
    pub fn new_perm_zettle(title: String, content: String) -> PermZettle {
        let contents: &String = &content;
        let tags: Vec<String> = tags(contents.to_string());
        PermZettle {
            title: title,
            content: contents.to_string(),
            tags: tags,
            }
        }

    }
