#[derive(Debug)] // for when you want to debug certain things
struct Rectangle {
    width: u32,
    height: u32,
}

// This is an implementation block for the rectangle
// Everything inside this block will be associated with the Rectangle type
// instances of this type can call the methods inside.
// &self is always the first parameter.
// The & makes it a borrow, so it borrows the Self instance.
// These methods can take ownership of self, borrow self immutably or borrow self mutably.
// All the methods or Rectangle should be put into the impl Rectangle

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is some sort of getter for Rust, when you name a method after the name of
    // one of the fields, than you should only do something with that field.
    fn width(&self) -> bool {
        // This method returns true if the width of the rectangle is greater than 0 and
        // returns false when the width is 0 or lower than 0
        self.width > 0
    }

    // Getters are useful because you can make the field private but the method public,
    // So you can still read the value, but not change it.

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Rust has the feature that when you call rect2.can_hold(&rect3), you don't automatically
    // take ownership from rect2, you can choose to if you want to by using self instead of
    // &self, but you don't have to do (&rect2).can_hold(&rect3), rust has syntax for this
    // you can just do rect2.can_hold(&rect3) and within the method you can use self
    // however you want.

    // All functions defined within an impl block are called associated functions
    // You can define functions within those blocks that don't have self as there first
    // parameter, so these are not methods, because they don't need an instance of the type
    // to work with.

    // These are mostly used for constructors that will return a new instance of the struct.
    // You could call a function like this f.e. new. But you don't have to, an example below
    // is square, which returns a square that of the value you just gave it.

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // The self in this associated function is an alias for the type that is right after
    // the impl, so in this case, Rectangle. You can see how to make one in main.

    // Multiple impl blocks of the same Type are possible and will be discussed
    // further in chapter 10.
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // this is method syntax for calling the area method on rect1.
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let _sq = Rectangle::square(3);
}