// Libs
use std::io::{stdin, Write};

use researcher::search_by;
use response_handler::get_results_from_res;

// Modules
mod google_entry;
mod researcher;
mod response_handler;

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
    let response = search_by(&users_input).expect("Couldn\'t complete the search.");

    // Get the results.
    let results =
        get_results_from_res(&response.text().expect("Couldn\'t load the request's body."));
    for result in results.iter() {
        println!("{result}")
    }
}
