fn main() {
    /*
    Iterating, loops, etc:
    Rust has 3 ways to iterate:
    loop, while and for. Here they are explained in a bit more detail.

    loop:
    Loop gives you a never ending loop, which if necessary you can stop with ctrl-c.
    f.e.:
    loop {
        println!("again!");
    }

    Loop can use the break and continue statements
    - break stops the loop right there on the spot
    - continue skips the remaining code in this iteration and goes straight to the
    next iteration of the loop.

    One of the uses of a loop is to retry an operation you know might fail,
    such as checking whether a thread has completed its job.
    If you want the result of the loop in a variable than you can use it like this:
    */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    /*
    You give the result of the loop (in this case 20) back by putting the thing that you want
    get out of the loop right after the break statement, in this case counter * 2.
    */

    /*
    Loop labels, this is something I have not seen before, you can label the loop.
    When you have a loop in a loop and you break, you only break the loop that is inside the loop.
    You cannot break the outer loop with the same break. With a loop label you can specify which
    loop you want to break or continue. The loop begin with a single quote, here is an example:
    */

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /*
    While loops:
    This is a conditional loop which also goes on forever until the condition isn't met anymore.
    Here is an example:
    */
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    /*
    For loop:
    I think everybody knows how this works, here is an example:
    */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    /*
    Here is what the while loop of before would look like if you put it in a for loop.
    (1..4) resembles 1 to 4 here so 1, 2 and 3, NO 4 !!! Than rev() reverses them.
    So this would have been the same as: for number in [3, 2, 1] {
    */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
