fn main() {
    // Generics is also the business of extracting functions to remove boilerplate code.
    // Say you have the functions here below: largest_i32 and largest_char, having a
    // function for each separate type is a bit unnecessary, so we can combine this into
    // one function with generics, which is called largest.
    // The header of this function will look like this: fn largest<T>(list: &[T]) -> &T {
    // But say you now make a function with the same body as the other 2 and that as a header
    // you will get an error. This is because you can only use types T that can implement the
    // PartialOrd trait, which means types of whose values can be ordered.

    // You can also define structs with Generics, for example to hold the x and y coordinates of
    // any type, the Point struct here below.
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // In this case in Point<T>, x and y are both of type T, which has to be the same type here.
    // Which means that this code won't compile
    // let wont_work = Point { x: 5, y: 4.0 };
    // If you do want that code to compile you should make a Point2 with two types like so:
    let _will_work = Point2 { x: 5, y: 4.0 };

    // You can use as many generics as you want, but if you need more than 3, your code probably
    // needs some restructuring.

    // Generics are also widely used in enums, for example Option or Result has Generics.
    // These are Option and Results:
    /*
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    // The Option<T> enum is generic over type T and has two variants: Some and None, where some
    // holds one value and None doesn't hold a value. By using the enum you can express the
    // abstract concept of an optional value, and because Option<T> is generic, you can use
    // this abstraction no matter what the type of the optional value is.

    // The result is generic over 2 types and has two variants which each hold a type. This makes
    // it convenient to use anywhere we have an operation that might succeed or fail.

    // You can also use generics in method definitions:
    // take look at x in the impl of Point.
    // Bij declaring T as a Generic type after impl, Rust can identify that the type in
    // the angle brackets in Point is a generic type rather than a concrete type.

    // You can also specify constraints on generic types when defining methods on the type.
    // We could, for example implement methods only on Point<f32>, this is also specified below.
    // The instances where Point<T> is not Point<f32> will not have this method available.

    // Generic type parameters in struct definition aren't always the same as those you use in
    // that same structs method signature X1 and X2, and, X2 and Y2 are mixed below for a
    // more clear example.
    // The method creates a new Point instance with the x value from the self Point of type X1
    // and the y value from the passed-in Point of Y2.
    // Here is some code for it:
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mix_up(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // What happens here is the following:
    // p1 is defined which is a Point3 that has an i32 as x and f64 as y.
    // p2 is defined which has a string slice for x and a char for y.
    // mix_up gets called on p1 with p2 as argument, this will get saved in p3.
    // This causes p3 to have an i32 for x, because it come from p1 and
    // a char for y because that came from p2.
    // So the macro will print p3.x = 5, p3.y = c.

    // Now for the part I have been waiting for, Performance:
    // It doesn't cost anything more! That's all folks.

}

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mix_up<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}


