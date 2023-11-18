// Libs
use crate::google_entry::GoogleEntry;

// Functions
/**
 * A method to get the results from the response.
 */
pub fn get_results_from_res(res_body: &str) -> Vec<GoogleEntry> {
    // Parse the request's body.
    let body = tl::parse(res_body, tl::ParserOptions::default()).unwrap();

    // Read the first 5 results.
    let pages = body.query_selector("div.fP1Qef").unwrap();
    let mut entries: Vec<GoogleEntry> = vec![];
    pages.for_each(|page| {
        let source_tag = page.get(body.parser()).unwrap().as_tag().unwrap();
        entries.push(GoogleEntry::new(body.parser(), source_tag));
    });

    entries
}
