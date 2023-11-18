// Libs
use std::fmt::Display;

use colored::Colorize;
use tl::{HTMLTag, Parser};

// Structs
pub struct GoogleEntry {
    link: String,
    title: String,
}

impl GoogleEntry {
    pub fn new(parser: &Parser, source_tag: &HTMLTag) -> GoogleEntry {
        let link = get_link_from_source(parser, source_tag);
        let title = get_title_from_source(parser, source_tag);
        GoogleEntry { link, title }
    }
}

impl Display for GoogleEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# [{0}] \n└ Link: {1}\n\n",
            self.title.green(),
            self.link.yellow()
        )
    }
}

// Functions
/**
 * A method to get the link from the entry.
 */
pub fn get_link_from_source(parser: &Parser, source_tag: &HTMLTag) -> String {
    // Get and treat all the html tree.
    let a_node = source_tag
        .query_selector(parser, "a")
        .unwrap()
        .next()
        .unwrap();
    let a_tag = a_node.get(parser).unwrap().as_tag().unwrap();
    let attr = a_tag.attributes().get("href").unwrap().unwrap();

    // Remove the "?q="
    attr.as_utf8_str().replace("/url?q=", "").to_owned()
}

/**
 * A method to get the title from the entry.
 */
pub fn get_title_from_source(parser: &Parser, source_tag: &HTMLTag) -> String {
    // Return the unique a node from a page.
    source_tag
        .query_selector(parser, "div.vvjwJb")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser)
        .into_owned()
}
