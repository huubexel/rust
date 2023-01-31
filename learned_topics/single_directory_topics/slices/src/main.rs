fn main() {
    /*
    Slices let you reference a contiguous sequence of elements in a collection rather than the
    hole collection.
    To learn this we are going to solve a small programming problem:
    write a function that takes a string of words, separated by spaces and returns the first
    word it finds in that string. If the string doesn't finds a space in the string,
    the string must be a whole word, so return that.
    */

    /*
    let mut s = String::from("hello world");
    let _word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    */
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    /*
    Doing all the above would be really tedious, luckily Rust has string slices.
    Rather than pointing to the whole String you can point to parts of the String.
    Like so:
    */
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    /* The slice data structure stores the starting position and the length of the slice. */
    /*
    In the case of f.e. world, world would be a slice with a pointer to the byte at index 6
    of variable s with the length value of 5. A pictures of this is in the google drive doc.
    */
    // with rust syntax if you want to start at index 0, you can drop the 0.
    //let slice = &s[0..2];  This,
    //let slice = &s[..2];   is the same as this.

    // When your slide includes the last byte of the string, you can drop the trailing number
    // Say that the String is Hello:
    // let slice = &s[3..5]; This,
    // let slice = &s[3..];  is the same as this.

    // If you want the entire string you can drop all numbers, again your String = hello:
    // let slice = &s[0..5]; This,
    // let slice = &s[..];   is the same as this.

    // now the first_word function becomes a lot easier, let's call it first_word2

    let s2 = String::from("hello world");
    let _word= first_word2(&s2);

    // Note, slicing only works with valid UTF-8 characters,
    // Characters that are multiple bytes cannot be sliced.

    // Now making a function like second_word would be quite easy as well.

    // You can refine this process even more by taking the slice as the input if you want to.
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    // You can also slice other things than just strings, this slicing works exactly the same.
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

/* You should start with this */
//fn first_word(s: &String) -> ? returns the first word of a reference to String s.

// We have not learned a way to do anything with the part of a string, so for now
// lets return the index at which the first space is located.
fn first_word(s: &str) -> usize {
    /* The as_bytes() converts the String to an array to iterate over,
    it cannot do this over a String */
    let bytes = s.as_bytes();

    // iter returns each element in a collection
    // enumerate wraps each result of iter as part of a tuple.

    // i == the index number, &item is the single byte in the tuple (the char).
    // Because you get a reference from .iter().enumerate() you use the &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // if the character is a space, b stands for byte. (only possible with u8)
            return i; // return the index of the space
        }
    }

    s.len() // otherwise return the length of the word.
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
