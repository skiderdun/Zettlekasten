// module for creating and managing zettels
// zettels are the basic unit of the zettelkasten
// they are indevidual notes that are linked together
// there are two types of zettels: liturature, and permanent
// liturature zettels are notes taken from a book or article
// permeant zettels are notes that are created by the user and are not linked to any external source

pub mod Zettle {
    use serde_json::Value;
    use url;
    use url::Url;
    use reqwest::{self, Response, Client};
    use serde_json;


    // liturature zettle struct
    pub struct LitZettle {
        pub citation: Vec<String>,
        pub doi: url::Url,
        pub content: String,
        }
    
    // function to access Crossref REST API to get Liturature Zettle citation data
    // takes a doi as a string and returns a citation vector
    pub fn get_citation(doi: String) -> Vec<String> {
        // create a vector to store the citation data
        let mut citation: Vec<String> = Vec::new();
        // create a url string from the doi and parse it into a url
        let doi_url: String = format!("https://api.crossref.org/works/{}", doi);
        let doi_url: Url = Url::parse(&doi_url).unwrap();
        // create HTTPS client
        let client: Client = reqwest::Client::new();
        // send a GET request to the doi url
        let mut res: Response = client.get(doi_url).send().unwrap();

    }
        

}