fn main() {
    /* This will only work if you use the mut here, otherwise it will crash
    let x = 5;  --> this would crash, you can only assign a value to an immutable once. */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /*
    With mut you make it mutable, a variable is immutable by default, so it is only
    mutable when you put mut in front of the variable.
    */

    /*
    Constants are also immutable variables, the difference is that constants
    can never become mutable, they are always immutable.
    You can make a constant with 'const' instead of let.
    The data type of a constant must be annotated, see example.
    You can have constants in every scope including the global scope.
    Constants have to be set during compile time, they cannot be set during runtime.

    The rust naming convention for constants is all caps with underscores between words.

    Since a constant has to be set during compile time it does not matter whether you type
    60 * 60 * 3 or 10800 here. It is not calculated during runtime so it does not cost any more
    time to have 60 * 60 * 3 here. For this reason use 60 * 60 * 3 here because it is more readable.

    It is useful to have one place in your code where you have all these constants so that
    if you need to find them you know where they are and
    it does not matter how many there are because you don't need to calculate these during runtime.
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are: {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    /*
    Shadowing, declaring a variable with the same name as a variable that was declared before.
    Say you have the variable guess and you put another value into it somewhere
    down the line than the first guess is shadowed by the second guess.
    The compiler will from now on only find the second guess.
    */

    let x = 5;

    /* you declared a new variable here so mutable is not needed */
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of guess in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*
    Sometimes you need to shadow because you cannot use mutable
    Say you get input from a user and you want to count the amount of spaces.
    Than this would be the way:
    */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("You typed {spaces} spaces");

    /*
    The following will not work:
    let mut spaces = "   ";
    spaces = spaces.len();

    This will not work because spaces is a string in the first line and in the second line it is
    an integer. You are allowed to mutate the string but you are not allowed to make it into
    another datatype and on the second line, spaces is an integer. spaces.len() won't work here.
    */
}
