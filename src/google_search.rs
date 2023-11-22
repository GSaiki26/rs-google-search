// Libs
use crate::google_entry::GoogleEntry;
use reqwest::blocking::Client;

// Functions
/**
 * A method to search in the Google about the requested term.
*/
pub fn search_by(sources: &mut Vec<GoogleEntry>, term: &str) -> Result<(), reqwest::Error> {
    // Create the url and then do the request.
    let url: String = String::from("https://google.com/search?q=") + term;
    let body = Client::new().get(url).send()?.text()?;

    // Treat the response and then return the google entries.
    let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
    let pages = dom.query_selector("div.fP1Qef").unwrap();
    pages.for_each(|page| {
        let source_tag = page.get(dom.parser()).unwrap().as_tag().unwrap();
        sources.push(GoogleEntry::new(dom.parser(), source_tag));
    });

    Ok(())
}
