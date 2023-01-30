use std::net::IpAddr;

fn main() {
    // the panic! macro occurs when bad things happen in the code, it can happen in 2 ways:
    // by taking an action that causes the code to panic, f.e. accessing an array past the end,
    // or by calling it explicitly.
    // Via an environment variable you can have Rust display the call stack when a panic occurs.
    // By default, if panic occurs, rust starts unwinding which is walking back the stack and
    // cleaning up the data from each function it encounters, this however takes some time.
    // You can also choose that Rust immediately aborts, Rust just immediately stops.
    // The operating system will have to clean up that mess, aborting is useful if you need to
    // make your binary as small as possible.
    // Say you want to abort on a panic in release mode, you have to do something like this:
    // [profile.release]
    // panic = 'abort'
    // In the Cargo.toml file.

    // Lets try calling panic!
    // panic!("crash and burn");

    // In the error, there are 2 lines that show the error,
    // The first one shows the panic message and the place where the panic occurred.
    // The second one shows the traceback of the places its been.

    // Now let's try it but not with the macro
    let _v = vec![1, 2, 3];

    /*println!(v[99]);*/

    // This will also fail but because of reasons you don't want to.


    // Now there is one question left: when should you use panic!? And when should you return
    // the Result? With panic! you give no option, the code just fails, when you use Result,
    // you give the code options. You could btw still call panic when choosing the Err of Result
    // so using the Result is the default option.
    // When prototyping or testing it is more appropriate to use panic! instead of Result.
    // When you want to clearly explain something, panic! is a nice way to handle errors, because
    // you want to be extremely clear what will happen. This is why things like unwrap and expect
    // are very useful when prototyping, they leave clear markers in your code for when you are
    // ready to make your program more robust.
    // Also use panic! for testing, if a method call fails, you want the whole test to fail
    // To make more clear what happened you can use expect()

    // You can also use unwrap or expect when you have some understanding of something that the
    // compiler does not, where you are 100% sure the thing you are about to do will not fail.
    // With expect you can document the reason why you think the Err variant will not appear.
    // Take a look at this code:
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    // We know this won't fail because we have hardcoded the IpAddress, so it will exist.
    // But because Rust is really save, you still have to handle the Result, well you can do
    // this with expect and write in the expect why you think it shouldn't fail.
    // If, in the future, you get the variable from say an input, and that could go wrong,
    // than the expect should be swapped out for something more robust.

    // Your code should panic if it could end up in a bad state. This is when some assumption
    // guarantee, contract or invariant has been broken. f.e. when invalid-, contradictory- or
    // missing values are passed to the code plus one more of the following:
    // - The bad state is something that is unexpected, as opposed to something that will likely
    // happen occasionally, like f.e. the user entering the data in the wrong format.
    // - The code after this point needs to rely on not being in this bad state. Rather than
    // checking for the problem at every step.
    // - There's not a good way of encoding the information in the types you use.

    // panic! is also valid when calling extern code which returns an invalid state, which
    // you cannot fix.
    // When failure is expected, you can use the Result.

    // When your code could put the user at risk if it's called using invalid values,
    // you should verify your code and panic when it is not valid. This is for safety reasons.

    // Some functions have contracts, their behavior is only guaranteed if the inputs meet the
    // requirements for the function. Panicking when this contract is violated makes sense because
    // this will create a called side-bug which you don't want to handle, they should just call
    // the function differently or with different values. This panic! should be explained in the
    // API documentation for the function.

    // Luckily Rusts type system already handles a lot of errors, when you expect a certain type
    // (a type different from Result or Option), and you give it that type, you already know that
    // that you have a valid type, which will most likely be a valid value here.
    // Code trying to pass nothing to the function won't even compile. So the function doesn't have
    // to check for that case at runtime. Another case is using an unsigned integer like u32,
    // this way you are 100% sure that the parameter is never negative.

    // This may sound weird, but you can also use i32 instead of u32 to have a way of saying hey
    // you cannot use negative numbers, this will than be communicated to the user instead of crash.

    // What you can also do is what is done in the code below.
    // First a struct is defined with a field named value that holds an i32, this is where
    // the number will be stored.
    // Than you have a function named new on Guess that creates instances of Guess values.
    // The new function is made to have one parameter (value) of type i32 and return a guess.
    // The code in the body tests if the value is within the borders that are accepted,
    // if the value doesn't pass this test, panic! gets called. This will alert the programmer
    // who is writing the calling code that a bug needs to be fixed. Cause in this case, creating
    // a guess that is not between 1 and 100 would violate the contract that Guess::new() is
    // relying on. The use of panic! here should be discussed in the documentation of the function
    // or library. If the value does pass, a Guess is created with the value inside.

    // Next a method named value that borrows self and returns an i32 --> getter. This method is
    // needed because the value field of Struct is private. This is important because the
    // value field using the Guess struct isn't allowed to set the value directly. Because code
    // outside the module must use the Guess::new function to create an instance of Guess.
    // This is how we ensure that there is no way a Guess has a value that hasn't been checked
    // by the conditions in the Guess::new function.

    // A function that has a parameter or returns only numbers between 1 and 100 could then
    // declare in its signature that it takes or returns a Guess rather than an i32 and, wouldn't
    // need to do any additional checks in it's body.

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
