use std::collections::HashMap;

fn main() {
    // If you want to use the HashMap, you do have to import it from the std library.

    // The hashmap has a lot of standard features already implemented, take a look at
    // those in the standard library.

    // You make a hashmap with new() and you insert stuff with insert(key, value), f.e.:
    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // A hashmap stores the data on the heap.

    // Just as with a vector, all the keys of the HashMap must be the same type and the values
    // also must be the same type.

    // With the get() method you can get the value from the HashMap by providing it with the
    // key, like so:
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0); // score will be 10.

    // The get method returns an Option<&V>, which means that if there is no value for the
    // provided key in the HashMap than get will return None. Here the option gets handled by
    // the copied() to get an option<i32> instead of an option<&i32>.
    // Than unwrap_or() to set score to 0 if scores doesn't have an entry for that key.

    // You can iterate over a HashMap in the following fashion.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Values that have the copy trait get copied to a hashmap.
    // The ownership for values that do not have this trait, like String, get transferred to
    // the hashmap:
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // The field_name and field_value will belong to the hashmap
    // If you try to use them anywhere else you will get an error

    let int_key: i32 = 9;
    let int_value: i32 = 200;
    let mut map_i32: HashMap<i32, i32> = HashMap::new();
    map_i32.insert(int_key, int_value);

    println!("{int_key}");
    // Because i32 has the Copy trait, the values will get copied in the HashMap
    // And you can still use them afterwards.

    // You can also use a reference of a value in the hashmap, the values which the
    // references point to have to be at least as long as the hashmap is valid.

    // Just as any other Hashmap, the key is unique. You can do a few things with the
    // new data at which a key is already set in the Hashmap:
    // Overwriting a value: If you insert a value on a key that already exists in the hashmap
    // it will be overwritten.
    map_i32.insert(9, 450);
    let value: i32 = map_i32.get(&9).copied().unwrap_or(0);
    println!("{value}"); // This will print 450 and not 200, 200 is gone here.

    // If the key already exists, don't insert it and keep the original value.
    // If the key does not exist, insert the key and the value:
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);
    // entry takes the key you want to check as a parameter and returns an enum called Entry.
    // This represents a value that or might not exist. The or_insert() method returns a mutable
    // reference to the value for the corresponding Entry key if that exists. If it does not exist
    // it inserts the parameter as the new value for this key and returns a mutable reference
    // to the new value, this plays nicely with the borrow checker and is much better than trying
    // it ourselves.
    // In the two lines above the comment block, the first call of entry will insert the key for
    // the yellow team with the value 50 because the yellow team doesn't have a value already.
    // The second call will not change the Hashmap because there is already a value for the
    // Yellow key in the HashMap.

    // Updating a value based on the old, already set, value. Looking up a value of a key and
    // updating it if necessary. Example:
    let text = "hello world wonderful world";

    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map3);
    // The world key has value 2 and the other values have 1 here.
    // The split_whitespace returns an iterator over sub-slices separated by whitespace.
    // The or_insert() returns a mutable reference to the value for the specified key.
    // Here that gets stored into count, in order to access count, you have to dereference it
    // with *
    // The mutable reference goes out of scope at the end of the for loop, so this is allowed.

    // The standard Hashmap is DDOS attack proof, but for that you do lose some speed, so
    // if you want to use some quicker or slower hashmaps, use the crates.io to search for
    // some crates to work with.
}
