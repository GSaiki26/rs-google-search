// Libs
use std::{
    fs::File,
    io::{stdin, Write},
    process, vec,
};

use clap::Parser;
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
fn get_selected_source(mut sources: Vec<GoogleEntry>) -> GoogleEntry {
    // Get the sources result.
    for source in sources.iter().enumerate() {
        println!("[{0}] {1}", source.0, source.1);
    }

    // Asks by the wanted source until get a valid option.
    let mut selected_source: Option<GoogleEntry> = None;
    for _i in 0..11 {
        let mut users_input = String::new();
        get_user_input("Select a source: ", &mut users_input);
        // Check if the input is valid.
        match users_input.trim().parse::<usize>() {
            Ok(index) if index < sources.len() => {
                let source = sources.remove(index);
                selected_source = Some(source);
                break;
            }
            _ => {
                println!("{}", "Invalid option. Please try again.".red());
                users_input.clear();
                continue;
            }
        }
    }

    match selected_source {
        Some(entry) => entry,
        None => {
            eprintln!(
                "Couldn\'t select the source due to max attempts. Considering using -s flag."
            );
            std::process::exit(1);
        }
    }
}

fn get_user_input(text: &str, buf: &mut String) {
    // Get the user's input.
    print!("{}", text);
    std::io::stdout()
        .flush()
        .expect("Couldn\'t flush the terminal.");
    stdin().read_line(buf).expect("Couldn\'t read the input.");
}

/**
 * A method to print on the screen the current configuration.
*/
fn show_info(args: &Config) {
    // Print the informations.
    println!("{}", "[RUST GOOGLE SEARCH]".green().bold());
    println!(
        "{}{}",
        "* Current selector defined: ".yellow(),
        &args.css_selector
    );
    let output = args
        .output
        .as_ref()
        .map_or_else(|| String::from("stdout"), |o| o.clone());

    println!(
        "{}{}\n\n",
        "* Current output path defined: ".yellow(),
        output
    );
    println!("Searching by: {}...", args.search_term);
}

// Main
fn main() {
    // Get the CLI arguments.
    let args = Config::parse();
    show_info(&args);

    // Search by the term on the google.
    let mut sources: Vec<GoogleEntry> = vec![];
    search_by(&mut sources, &args.search_term).expect("Couldn\'t complete the search.");

    // Check if the source flag was used. Otherwise get the selected source.
    let mut source = if let Some(index) = args.source {
        if index < sources.len() {
            sources.remove(args.source.unwrap())
        } else {
            eprintln!(
                "The provided source index was not found. Try to decrease the source number."
            );
            std::process::exit(1);
        }
    } else {
        get_selected_source(sources)
    };

    // Get the main content of the page.
    let source_content = source.get_content();
    if source_content.is_none() {
        process::exit(1)
    };

    println!("Request completed.");

    // Check if the output is defined to print it on the screen.
    if args.output.is_none() {
        return println!("{0}", source_content.unwrap());
    }

    // Try to write the result on the file.
    if let Ok(mut f) = File::create(args.output.unwrap()) {
        f.write_all(source_content.unwrap().as_bytes())
            .expect("Couldn\'t write on the file");
        return println!("Content written in the file.");
    }
    panic!("Couldn\'t create the output file. Please check permissions and typos.");
}
