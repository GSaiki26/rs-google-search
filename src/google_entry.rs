// Libs
use std::fmt::Display;

use clap::Parser;
use colored::Colorize;
use reqwest::blocking::Client;
use tl::{HTMLTag, Parser as TlParser};

use crate::config::Config;

// Structs
pub struct GoogleEntry {
    pub link: String,
    pub title: String,
    content: Option<String>,
}

impl GoogleEntry {
    pub fn new(parser: &TlParser, source_tag: &HTMLTag) -> GoogleEntry {
        // Get the link and the title.
        let (mut title, mut link) = (String::new(), String::new());
        Self::define_title(&mut title, parser, source_tag);
        if Self::define_link(&mut link, parser, source_tag).is_none() {
            eprintln!("The source tag doesn't have a link or a title.");
            std::process::exit(1);
        }

        GoogleEntry {
            link,
            title,
            content: None,
        }
    }

    /**
     * A method to extract the main content from some source.
     */
    pub fn get_content(&mut self) -> Option<&str> {
        // Check if the content already is defined.
        if self.content.is_some() {
            return Some(self.content.as_ref().unwrap());
        }

        // Get all the P and span elements.
        let mut body = String::new();
        self.get_source_body(&mut body)?;
        let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();

        // Check if the main element exists.
        let args = Config::parse();
        let tags = dom.query_selector(&args.css_selector)?;
        let mut content: String = String::new();

        for tag in tags {
            let text = tag.get(parser)?.inner_text(parser);
            content.push_str(&format!("* {0}\n\n", text.trim()));
        }

        if content.is_empty() {
            println!("The current css_selector couldn\'t find anything to scrap on the source.");
            return None;
        }

        self.content = Some(content);
        return Some(self.content.as_ref().unwrap());
    }

    /**
     * A method to define the link by wraping it from the HTML.
     */
    fn define_link(buf: &mut String, parser: &TlParser, source_tag: &HTMLTag) -> Option<()> {
        // Get and treat all the html tree.
        let a_node_handler = source_tag.query_selector(parser, "a")?.next()?;
        let a_tag = a_node_handler.get(parser)?.as_tag()?;
        let link = a_tag.attributes().get("href")??;

        // Remove the Google params from the link
        buf.push_str(
            link.as_utf8_str()
                .replace("/url?q=", "")
                .split("&amp")
                .next()?,
        );
        Some(())
    }

    /**
     * A method to define the title by wraping it from the HTML.
     */
    fn define_title(buf: &mut String, parser: &TlParser, source_tag: &HTMLTag) -> Option<()> {
        // Return the unique a node from a page.
        let title_element = source_tag.query_selector(parser, "div.vvjwJb")?.next()?;

        buf.push_str(&title_element.get(parser)?.inner_text(parser));
        Some(())
    }

    /**
     * Get the body's from the source.
     */
    fn get_source_body(&self, buf: &mut String) -> Option<()> {
        // Do the request to get the body.
        println!("Doing request to: {}", self.link);
        let response = Client::new()
            .get(&self.link)
            .send()
            .expect("Couldn\'t connect to the selected source.");
        buf.push_str(&response.text().unwrap());
        Some(())
    }
}

impl Display for GoogleEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print the entry.
        write!(f, "# [{0}]", &self.title.green())
    }
}
