fn main() {
    /*
    Rust must know what data type a variable is at compile time,
    Sometimes it is really clear but if it is not and you do want to make sure that the
    compiler gets the right type than you have to do something like this:
    to parse 42 we need to pass an integer, so you have to specify which type you want to parse
    otherwise nothing can be parsed and the compiler gives an error.
    */
    let _guess: u32 = "42".parse().expect("Not a number!");

    /*
    You have 2 data types: scalar and compound

    Scalar:
    represents a single value, there are 4 primary scalar values:
    integer, floating-point, boolean, character.

    Integer:
    i8 --> signed integer. Goes from -128 to 127.
    u8 --> unsigned integer. Goes from 0 to 255.
    You have 8, 16, 32, 64, 128 and usize and isize, which is 32 or 64 bits.

    You have 2 little tricks here:
    1: you can write 1000000 as 1_000_000 to make it more readable. Etc.
    2: say you want a number to be a specific integer type, you can assign that like for example
    you want the number 58 to get 16 bits than you type 58u16.

    If you ever get the possibility of overflow, use one of these:
    wrapping_*, checked_*, overflowing_*, saturating_* (you can look these up).
    */
    println!("{}", 58u16);

    /*
    Floating point:
    You have 2 variants f32 and f64 where f64 is 2 times more precise, with almost the same speed.
    If you do not specify rust will use f64 but if you want to use f32 you could use the f32:
    */
    let _y: f32 = 3.0; // f32

    /*
    How do numeric operations work?
    addition: 5 + 10
    subtraction: 95.5 - 4.3
    multiplication 4 * 30
    remainder: 43 % 5
    division is a bit more tricky:
    quotient: 56.7 / 32.2 --> will give you a f64
    truncated: -5 / 3 --> will give you an int, in this case -1
    */

    /*
    Boolean:
    There are 2 values: true and false. They are one byte in size.
    You can once again not set it if it is really clear, or set it like this:
    */
    let _b: bool = false;

    /*
    Character:
    A character is specified with single quotes (a string is specified with double quotes),
    a lot of things are included in char but not everything. It is 4 bytes long and you can
    either just set it with the '' single quotes or set it with char like this:
    */
    let _c: char = 'k';

    /*
    Compound types:
    Can group multiple values into one type. Rust has 2 build in primitive types: Tuples and arrays.

    Tuple:
    Groups together a number of values with a variety of types into one variable.
    Tuples have fixed sizes, once they are made they cannot shrink or grow.
    You create a tuple like here below, the type annotations are optional.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1); // single compound element.

    /* To get the values out of the tuples, you can use pattern matching like this: */
    let (_g, h, _i) = tup;
    println!("The value of h is: {h}");

    /* Breaking the tuple into separate parts like this is called destructuring */

    /* You can also sort of index the tuple with the period . this like so: */
    let _five_hundred = tup.0; // first index
    let _six_point_four = tup.1; // second index, etc.

    /*
    When a tuple is empty it is called a unit and the value and corresponding type are both
    written like () this is an empty value or empty return type, expressions return the unit
    if they don't return anything else.
    */

    /*
    Array:
    An array is also a number of values, however, every value must have the same type to be
    able to fit in the array. Arrays in rust have a fixed length.
    Arrays are useful if you want your data allocated on the stack instead of on the heap.
    An array is different from a vector in that its length is fixed, a vectors length
    may grow or shrink. This is what an array looks like:
    */
    let a: [i32; 5] = [1, 7, 4, 9, 2];
    /* First the type of the values, than the amount of values and than the values itself */

    /* You can also make an array with the same value for each element like so: */
    let _b = [3; 5]; // --> [3, 3, 3, 3, 3]

    /* You can access the array with indexing like so */
    let _fourth = a[3]; // the fourth index

    /* Rust is very memory save and say you want to access a non-existing index, it will
    check whether the length of the array is more or even what you want and will otherwise
    give an error, more on how to deal with those later on.
    */
}
