use std::env; // env has args() which can read command line arguments.

fn main() {
    /* Gets the Arguments given in by the user and put them in a Vector */
    let args: Vec<String> = env::args().collect();

    /* We do not save the &args[0] here because that is the name of our binary and
    we don't need that for this assignment. */
    let query = &args[1]; // the thing you want to get
    let file_path = &args[2]; // the file you will be searching in

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}


/* In this case we have chosen args(), args() does require you to use normal unicode.
so anything special like emoji's it cannot read and it will panic if something goes wrong
if you want to also be able to get special characters like emoji's, use args_os instead. */

/* Collect is one of the rare functions where you have to specify which you want as an income
so in this case we want a vector of strings. */


// dbg!(args); /* prints args using the debug macro */