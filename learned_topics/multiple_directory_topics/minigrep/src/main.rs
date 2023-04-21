use std::env; // env has args() which can read command line arguments.
use std::fs; // fs has functions to handle files

fn main() {
    /* Gets the Arguments given in by the user and put them in a Vector */
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    /* Opens a file and returns a Result<String> of the file's contents. */
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}


/* In this case we have chosen args(), args() does require you to use normal unicode.
so anything special like emoji's it cannot read and it will panic if something goes wrong
if you want to also be able to get special characters like emoji's, use args_os instead. */

/* Collect is one of the rare functions where you have to specify which you want as an income
so in this case we want a vector of strings. */


// dbg!(args); /* prints args using the debug macro */