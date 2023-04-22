use std::{env, process};

use minigrep::Config;

fn main() {
    /* Gets the Arguments given in by the user and put them in a Vector */
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

/* In this case we have chosen args(), args() does require you to use normal unicode.
so anything special like emoji's it cannot read and it will panic if something goes wrong
if you want to also be able to get special characters like emoji's, use args_os instead. */

/* Collect is one of the rare functions where you have to specify which you want as an income
so in this case we want a vector of strings. */


// dbg!(args); /* prints args using the debug macro */
