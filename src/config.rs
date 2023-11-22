// Libs
use clap::Parser;

// Structs
// A Rust application that search by terms on Google.
#[derive(Parser)]
pub struct Config {
    /// The term to be search.
    #[arg()]
    pub search_term: String,

    /// The css selector that'll be used to extract the main content from the source.
    /// By default, the <main> is used.
    #[arg(short, default_value_t=String::from("main"))]
    pub css_selector: String,

    /// The output path to write the content.
    /// If not defined, the stdout'll be used.
    #[arg(short, long)]
    pub output: Option<String>,

    /// The auto source to be selected.
    /// If not defined, the source'll be asked, requiring stdin.
    #[arg(short, long)]
    pub source: Option<usize>,
}
