fn main() {
    /* scope example */
    {                           // y is not valid here, it’s not yet declared
        let _y = "hello";      // y is valid from this point forward
        // do stuff with y
    }                          // this scope is now over, and y is no longer valid

    /* String type created with text from string literal */
    let _x = String::from("hello");

    /* Example with String type */
    let mut s = String::from("Hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `Hello, world!`

    {
        let _d = String::from("hello"); // d is valid from this point forward
        // do stuff with s
    }                                  // this scope is now over, and d is no longer valid
    // d doesn't exist here anymore, when I now print d, Hello, world! will be printed, not hello.


    let s1 = String::from("hello");
    let _s2 = s1;
    // now printing s1 would give an error because it is no longer seen as viable to Rust.

    // If you want to have a 'deep' copy of s1 you should use this.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // This still works because it is both on the stack, so copies are quick to make,
    // so writing clone would be a bit unnecessary here.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);



    // For using functions the ownership works the same.

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                              // ... and so is no longer valid here

    let x = 5;                           // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    // Here, x goes out of scope, then s.
    // But because s's value was moved nothing special happens.



    // See here what happens when a function has a return value.
    let _s1 = gives_ownership();                        // gives_ownership moves its return
                                                        // value into s1

    let s2 = String::from("hello");           // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);         // s2 is moved into
                                                        // takes_and_gives_back, which also
                                                        // moves its return value into s3



    // To be able to return more back than just the parameter itself, you can use
    // a tuple to return multiple things at once in rust.

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {                        // gives_ownership will move its
                                                        // return value into the function
                                                        // that calls it

    let some_string = String::from("yours");  // some_string comes into scope

    some_string                                         // some_string is returned and
                                                        // moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into scope

    a_string                    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
