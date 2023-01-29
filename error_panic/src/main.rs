
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
    let v = vec![1, 2, 3];

    println!(v[99]);

    // This will also fail but because of reasons you don't want to.

}
