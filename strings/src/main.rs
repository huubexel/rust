fn main() {
    println!("Hello, world!");

    // Naturally this is a mutable String in Rust.
    let mut _s = String::new();
    // This is an empty string, if you want it to have initial data you use the ::from(), like so:
    let mut _s1 = String::from("something");
    // Don't forget, double quotes is more than 1 char, single quotes is 1 char.

    // If you have some data that is a string but it not yet in a string type you can use to_string:
    let data = "initial content";
    let _s2 = data.to_string();
    let _s3 = "initial content".to_string(); // s2 == s3
    // Doing this gives it the display trait. The String also has this.

    // With the ::from(), you can get every type of input:
    let _hello = String::from("안녕하세요"); // this is valid!

    // You can update a String using the push_str() method on a mutable String, like so:
    let mut s4 = String::from("Hello, ");
    s4.push_str("World!");

    // push_str() does not take ownership of s6 here, otherwise you wouldn't be able to print
    // it's value on the last line.
    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s6 is {s6}");

    // You can also just push one character with the push() method:
    let mut s7 = String::from("lo");
    s7.push('l');
    println!("{s7}");

    // You can also concatenate 2 existing strings with the + operator.
    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let _s10 = s8 + &s9; // note s8 has been moved here and can no longer be used


    // You can also do more than 2 variables at once:
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");

    let _s14 = s11 + "-" + &s12 + "-" + &s13;

    // But after this it would be better to use format, which looks like this
    let s15 = String::from("tic");
    let s16 = String::from("tac");
    let s17 = String::from("toe");

    let _s18 = format!("{s15}-{s16}-{s17}");
    // And what is also very nice is with format! is that the macro doesn't take any
    // of the variables it takes in as parameters their ownership, these are all references.

    // Now unfortunately there is one thing shit, you cannot use String indexing in Rust:
    // This is invalid code: s1[0], this will not compile. There is a reason for this.
    // String is a wrapper around Vec<u8> which is logical because that is an expandable
    // array with numbers from 0 to 255 possibilities, enough for ASCII. However when you
    // start using other languages and write something like this: Здравствуйте you will
    // guess that that is 12 bytes long, nope it is 24bytes because it takes 2 bytes per
    // russian letter to fit it into UTF-8. So when you try to grab the д with s[1], you are
    // taking the second byte of the first letter. So you cannot index this, because not
    // everything is 1 byte in UTF-8.

    // You can however use [0..4] to get the first two letters of russian here, however
    // this is not good practice to do, especially when you use multiple languages on
    // your project.

    // The best way to slice a string is to iterate over it unfortunately. This like so:
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }
    // for these unicode scalar values, use the .chars() it separates them correctly so
    // you can easily iterate over them.

    // Than you also have some weird indian letters or something like japanese or chinese
    // for that, just use a crate.

    // If you would use .bytes() instead of .chars you would get the integers where the
    // letters are made up of.

}
