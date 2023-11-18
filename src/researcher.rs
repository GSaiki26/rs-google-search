// Libs
use reqwest::blocking::{Client, Response};

// Functions
/**
 * A method to search in the Google about the requested term.
*/
pub fn search_by(term: &str) -> Result<Response, reqwest::Error> {
    // Create the url and then do the request.
    let url: String = String::from("https://google.com/search?q=") + term;
    Client::new().get(url).send()
}
