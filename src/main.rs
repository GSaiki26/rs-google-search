// Libs
mod entry;
mod researcher;

use entry::GoogleEntry;
use researcher::search_by;

// Main
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Do the request.
    let response = search_by("Cats into the real world!")?;

    // Parse the request's body.
    let body = response.text().expect("Couldn\'t load the request's body.");
    let body = tl::parse(body.as_str(), tl::ParserOptions::default())?;

    // Read the first 5 results.
    let pages = body.query_selector("div.fP1Qef").unwrap();

    for page in pages {
        let tag = page.get(body.parser()).unwrap().as_tag().unwrap();
        let entry = GoogleEntry::new(body.parser(), tag);
        println!(
            "@@Title: {0}\nUrl: {1}\n\n",
            entry.get_title(),
            entry.get_link()
        );
    }

    Ok(())
}
