fn main() {
    // Vectors Vec<T> allow you to store more than one value in a single data structure.
    // These values are all put next to each other in memory.
    // These values all need to be the same. They are useful when you have a list of items
    // like lines of text or the prices of items in a shopping cart.

    // To make an empty vector (in this case with Type i32), you do this:
    let _v: Vec<i32> = Vec::new();
    // Now we naturally can only store int i32's in here. i32 is the default integer type
    // in rust, so say you want a vec with integers i32 than there is a shortcut for that:
    let _v2 = vec![1, 2, 3];
    // Rust can infer that the type is i32 so you won't have to specify that.

    // Updating a vector is rather easy, just use the following:
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    // Rust yet again recognizes i32 and will go with that from here on.

    // You naturally also want to get things out of the vector, you can use either
    // indexing or the get method for that:
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // As you can see it might be smart to use the Some and None here.
    // Rust has two ways to do this for a specific reason, you can do
    // it like the first way: let third: &i32 = &v4[2]; but if you do f.e.
    // let hundred: &i32 = &v4[99]; and there is no 100th index, there will be
    // an error. So you can than use the second option with get to always have a
    // solution. I would personally say that the first one is alright if you know
    // very well that you won't have an error because you do it in compile time.
    // Once you start doing things like this in runtime, I would use the second
    // one to be sure. The second option is way more user friendly.

    // You can also iterate over the values in a Vector:
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }
    // This loop gives immutable references to the items in the vector.
    println!();
    // If you want mutable references to the items in the vector, use this:
    let mut v6 = vec![100, 32, 57];
    for j in &mut v6 {
        *j += 50;
        println!("{j}");
    }

    // But say I really want to get more than one type in a vector? On paper
    // it is not possible with only a vector. But if you combine it with an enum
    // it is possible. An enum is only 1 type, the name of the enum, which means
    // you can do a little something like this.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // The vector row now has 1 type --> SpreadsheetCell and SpreadsheetCell
    // has 3 types so that's how you can have a Vector with multiple different
    // 'types' in it. There is one but, you do have to know exactly what types
    // the program will need from the enum, otherwise you will get a runtime error
    // You can use traits but those are not explained until chapter 17.
    // There is a hole lot more documentation on the Vec<T> which you can read at
    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    // One more quick tip, pop pops the last element from the Vector:
    v3.pop();

    // When you drop a vector, you also drop all it's elements naturally, so when
    // it goes out of scope, all elements and the vector itself get dropped.
}
