// Libs
use std::{
    io::{stdin, Write},
    process, vec,
};

use colored::Colorize;
use config::Config;

use google_entry::GoogleEntry;
use google_search::search_by;

// Modules
mod config;
mod google_entry;
mod google_search;

// Functions
/**
 * A method to get the user's input.
*/
fn get_user_input(asks: &str, buf: &mut String) {
    // Get the user's input.
    print!("{asks}");
    std::io::stdout()
        .flush()
        .expect("Couldn\'t flush the terminal.");
    stdin().read_line(buf).expect("Couldn\'t read the input.");
}

// Main
fn main() {
    // Get the user's input and then call google.
    let mut users_input = String::new();
    get_user_input("Search by: ", &mut users_input);
    let mut sources: Vec<GoogleEntry> = vec![];
    search_by(&mut sources, &users_input).expect("Couldn\'t complete the search.");

    // Get the sources result.
    for source in sources.iter().enumerate() {
        println!("[{0}] {1}", source.0, source.1);
    }

    // Asks by the wanted source until get a valid option.
    loop {
        // Check by parser Error.
        users_input = String::new();
        get_user_input("Select a source: ", &mut users_input);
        let index_chosen = users_input.trim().parse::<usize>();
        if let Ok(index) = index_chosen {
            // Check by out of range input.
            if index < sources.len() {
                break;
            }
        } else {
            println!("Error");
        }

        println!("{}", "Invalid option.".red());
    }

    // Get the source's body.
    let index_chosen = users_input.trim().parse::<usize>().unwrap();
    let selected_source = &mut sources[index_chosen];

    // Get the main content of the page.
    match selected_source.get_content() {
        Some(source_content) => println!("{0}", source_content),
        None => {
            eprintln!("Couldn\'t request the source.");
            process::exit(1);
        }
    };
}
