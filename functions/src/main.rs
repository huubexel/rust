/// Doc tags.
fn main() {
    println!("Hello, world!");
    /*
    Functions:
    You make a new function with fn name_of_function() {}
    Rust uses snake case for functions and variables so use this variable_name, not this
    variableName.
    This is how you would call another function:
    */
    another_function();

    /* I think it is the convention in rust to put the main function at the top of the page? */

    /* Naturally a function can have parameters like this in rust: */
    another_function_two(8);

    /* In function parameters you must declare the variable type, like in this case i32 */
    /* Here is an example with multiple parameters */
    print_labeled_measurement(5, 'h');

    /*
    Function bodies are made up of statements that optionally end in an expression,
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resulting value.
    When you use let, you are making a statement, the following is a statement:
    */
    let _y = 6;

    /*
    It is very important to remember that statements do not return values.
    This is why you cannot assign a let statement to another variable,
    this line would give an error:
    let x = (let y = 6);
    y does not return anything, so there is nothing for x to bind to.
    In languages like C you can x = y = 6, because of this rule you cannot do this in rust.
    */

    /*
    Expressions evaluate to a value, remind this as well.
    5 + 6 is an expression that evaluates to the value 11.
    let y = 6; The 6 here is an expression that leads to the value 6.
    Calling a function or macro is an expression.
    A new scope block created with curly brackets is an expression. f.e.:
    */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    /*
    Here the let y = and let x = are the statements and the rest are expressions
    Here the line x + 1 doesn't need a semicolon because it is not a statement, it is an expression.
    If you would have put a ; at the end of that line it would have been an expression and would
    not have returned anything. x + 1; gives an error here.
    */

    /*
    In rust you can return something in a function, as usual.
    The last expression that is in the end of a scope gets returned, so in the y scope above here,
    x + 1 is returned. In a function also the last expression in the scope is returned.
    However if you want to return something earlier (while using recursion for example) use the
    keyword return and than after it what you want to return.
    If you want to return something in a function use the arrow and after it you have to state what
    you want to return. The function called five is an example of this.
    */
    let x = five();
    println!("The value of x is: {x}");

    /*
    If you would want to use a bit more interactive function you can
    give the function a parameter.
    */
    let z = plus_one(9);
    println!("The value of z is: {z}");

    /*
    Say you would have used x + 1; in the plus_one function, than your function would have
    a statement (with an expression at the end of it) and would not return anything and would
    prompt an error. The error will be that the function returns a unit () and it should
    return an i32 because that is what you specified.
    */

    /*
    Alright this section is done, there is a section about comments in rust, i am not gonna
    make a dedicated project for that so here it is.
    You have 2 different comments:
    */
    // this one
    /* this one */

    /*
    We both know how they work, I have been using them here before, these are almost the same
    in most of the languages. I saw that you could use doc tags, they are on the first line of this
    document, you have to use /// on every line you want to write something.
    */

    // When actually writing code, maybe its smart to use this because the book doesnt mention
    // the CSS like comment.
}

fn another_function() {
    println!("Another function.");
}

fn another_function_two(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32  {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
