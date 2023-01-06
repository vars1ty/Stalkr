mod constants;
mod http;

use arguments::Arguments;
use constants::*;

/// Setup CLI Arguments.
fn setup_args() -> Arguments {
    let arguments = std::env::args();
    arguments::parse(arguments).expect("[ERROR] Failed parsing arguments!")
}

/// Startup function.
fn main() {
    let query = setup_args()
        .get::<String>("query")
        .expect("[ERROR] Missing --query [...]");

    for source in SOURCES {
        if !http::check_if_valid(source, &query) {
            continue;
        }

        // Valid
        println!("{}", source.replace("[user]", &query))
    }
}
