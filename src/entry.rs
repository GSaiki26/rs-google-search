// Libs
use tl::{HTMLTag, NodeHandle, Parser};

// Structs
pub struct GoogleEntry<'a> {
    parser: &'a Parser<'a>,
    tag: &'a HTMLTag<'a>,
}

impl<'a> GoogleEntry<'a> {
    pub fn new(parser: &'a Parser<'a>, tag: &'a HTMLTag) -> GoogleEntry<'a> {
        GoogleEntry { parser, tag }
    }

    /**
     * A method to get the link from the entry.
     */
    pub fn get_link(&self) -> String {
        // Get and treat all the html tree.
        let a_node = self.get_a_node();
        let a_tag = a_node.get(self.parser).unwrap().as_tag().unwrap();
        let attr = a_tag.attributes().get("href").unwrap().unwrap();

        // Remove the "?q="
        attr.as_utf8_str().replace("/url?q=", "").to_owned()
    }

    /**
     * A method to get the title from the entry.
     */
    pub fn get_title(&self) -> String {
        // Return the unique a node from a page.
        self.tag
            .query_selector(self.parser, "div.vvjwJb")
            .unwrap()
            .next()
            .unwrap()
            .get(self.parser)
            .unwrap()
            .inner_text(self.parser)
            .into_owned()
    }

    /**
     * A method to get the all the a tags from the entry.
     */
    fn get_a_node(&self) -> NodeHandle {
        // Return the unique a node from a page.
        self.tag
            .query_selector(self.parser, "a")
            .unwrap()
            .next()
            .unwrap()
    }
}
