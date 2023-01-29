use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::fs;

fn main() {
    // Most error aren't serious enough to panic. Say f.e. a file doesn't exist yet.
    // If you try to access it, it would crash the program, so you handle the error
    // and when it doesn't exist, you create the file.

    // This is the Result enum that you saw before, where T when Ok and E when Err
    //enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    //}
    // There are a lot of functions that implement this, so let's use one.
    let greeting_file_result = File::open("hello.txt");
    // As the IDE already suggests, the return type is a Result<File>
    // If the file exists this will return a File, std::fs:File
    // when it does not, it will return std::io:Error
    // So if File::open succeeds in the variable there will be an instance of Ok that contains
    // a file handle. In the case that it fails the value will be an instance of Err
    // that contains information about the kind of error that happened.

    // We need and want to handle both situation, a way to do this is with the match expression.
    //let greeting_file = match greeting_file_result { // quick example.
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening the file: {:?}", error),
    //};

    // Now say we don't want to immediately panic. Than we'll have to respond to specific errors.
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // The return value is error when error is valid. error has a kind() method to get
    // an ErrorKind value, this enum is provided by the std library and has to be included
    // It represents errors you might want to use, in this case NotFound.
    // This matches on if the file isn't found.

    // If for some reason you don't want the match expression this is an alternative.
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // This is a bit cleaner and cleans up the match statement you were using.

    // There are some shortcuts for panic, unwrap and expect.
    // Unwrap: if the Result is Ok, unwrap will return the value inside the Ok.
    // If the Result is Err, unwrap will call the panic macro, this is how to use it:
    let _greeting_file3 = File::open("hello.txt").unwrap();
    // Expect: does exactly the same thing as unwrap, but you have to give a message as parameter.
    // For this reason, just use expect.
    let _greeting_file4 = File::open("hello.txt").expect("This file does not exist yet");


    // Propagating errors:
    // When the code inside a function calls something that might fail, instead of handling the
    // error within the function itself, it returns the error to the calling code, so it can
    // decide what to do. This is propagating the error. It gives more of the control to the
    // calling code.
    // The function read_username_from_file uses propagation errors. When the Result is Ok,
    // it returns a String and when the Result is Err, it returns an io::Error
    // io::Error is chosen here because it is both the error types from File::open
    // and read_to_string. As you can see at the match, the Err gets returned with the return
    // statement. The error gets passed into e. If the match passes, a new username variable
    // gets made. Than another match gets called where the read_to_string gets filed on the file
    // handle in username_file to read the contents of the file into username. Here in the Err,
    // we don't need to say return because it is the last thing in the function.

    // This pattern of propagating error is so common in Rust that there is a shorthand for this
    // with the ? operator. The read_username_from_file_2 does exactly the same as the
    // read_username_from_file but now with the shorthand ? operator.
    // If the Result is Ok, return the String, otherwise the error. It works the same.

    // However there is a difference between the ? and the match operator.
    // error values that have the ? operator called on them go through the from function,
    // defined in the From trait in the std library, which convert the types of values.
    // The error type received is converted into the error type defined in the return type
    // of the function, in this case in io::Error. So you could adapt some code to return
    // another type of error here, see 9.2 for more info.
    // The ? eliminates a lot of boilerplate code.
    // You could even make the function shorter now by chaining functions together, version3.

    // There is an even shorter version of the code, version 4.
    // This reads a file into a string, is the most convenient way.

    // You can only use the ? operator when the return type of the function is Result,
    // so that it's compatible with this return. When you for instance use ? in this
    // main function you get an error saying that what you are doing is not good.
    // You have 2 options to not use the ? operator, 1. the function using the ? in
    // has to be compatible with the value you're using the ? on.
    // Or 2, use match or other technique to handle the Result.

    // You can also use the ? on an Option, as long as the return type of that function
    // is also Option, last_char_of_first_line is an example of this.

    // This function works as follows:
    // The codes takes the text string slice as argument and calls the lines method on it.
    // This returns an iterator over lines in the string.
    // The function needs the first line, so it calls next(). If the file is empty, next()
    // could return None, that is why we use ? here. If there is a first line, next will
    // return Some containing the string slice that is the first line.
    // So if everything is correct, you get the string from next, than chars gets called
    // which gives an iterator over the characters. Than last gets called because you want
    // only the last character. This is an Option because it is possible that the first string
    // is empty.

    // You can use a ? on a Result if the function returns a Result and you can use a ?
    // on an Option if the function returns an Option. But you can't use both at the same time.
    // If you do want to use both at the same time, you have to use the ok method on Result,
    // or the ok_or on Option to do the conversion.

    // So far the main function always returned a unit () there are restrictions on what
    // the main function can return.
    // Luckily it can return a Result<(), E>
    // So this would compile:
    /*
    fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
    */

    // The Box<dyn Error>> is a trait object, for now, it is any kind of error.
    // Because this is any kind of error, you can now use ? in main.
    // If the main function returns Result<(), E>, the program will exit with a value of 0,
    // if main returns Ok(()) and it will exit with a nonzero if main returns an Err

    // The main function may return any types that implement the std::process::Termination trait,
    // these contains a function report that returns an exit code. You can implement these,
    // specific traits yourself, consult the library for that.

}

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

