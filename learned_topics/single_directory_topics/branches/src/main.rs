fn main() {
    /*
    If statements:
    These look rather natural in rust, they look something like this
    if number < 5 {
        println!("condition is met");
    } else {
        println!("condition was false");
    }
    */
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    In rust the condition must be a bool, so a true false situation, this is not like in C
    where 0 == false and the rest == true.
    This code here would express an error:
    let number = 3;
    if number {
        println!("number was three");
    }

    Something like this would work:
    */
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    /* rust also has an else if statement for when multiple options are possible, f.e.: */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /* Rust executes the first thing from this block that is true and nothing more. */
    /* If you get a lot of else if statements you might want to use match, more on that later. */

    /* in rust you can use if statement inside of a let to determine the outcome of the let, f.e. */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /*
    In rust it is also the case that the outcomes resulting from the if else must be the same,
    which means that if you would've had this as let number, it wouldn't have worked:
    let number = if condition { 5 } else { "six" };
    It would give an error that it found an integer 5 and the else part must also be one.
    */
}
