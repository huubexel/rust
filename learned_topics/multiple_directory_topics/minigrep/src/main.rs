use std::env; // env has args() which can read command line arguments.
use std::fs; // fs has functions to handle files

fn main() {
    /* Gets the Arguments given in by the user and put them in a Vector */
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    /* Opens a file and returns a Result<String> of the file's contents. */
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

/* In this case we have chosen args(), args() does require you to use normal unicode.
so anything special like emoji's it cannot read and it will panic if something goes wrong
if you want to also be able to get special characters like emoji's, use args_os instead. */

/* Collect is one of the rare functions where you have to specify which you want as an income
so in this case we want a vector of strings. */


// dbg!(args); /* prints args using the debug macro */

/* Normally you would not want to use clone because the runtime performance will be worse,
but for now this will do. In your absolute final code you would want to use something
different in this situation. */