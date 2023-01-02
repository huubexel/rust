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

    println!("{}", s); // This will print `hello, world!`
}
