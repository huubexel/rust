fn main() {
    // To create an instance of the IpAddrKind V4 and V6 you have to use this:
    let _four = IpAddr::V4;
    let _six = IpAddr::V6;

    // Here you can also see that the IpAddr::V4() and "::v6()
    // are automatically constructors for the enum variants.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // You can call the function route with either variant
    route(home);
    route(loopback);

    // Enum method call.
    let m = Message::Write(String::from("hello"));
    m.call();


    // Null doesn't exist in Rust, the concept however does exist, Rust has
    // an enum that can encode the concept of a value being present or absent,
    // which is basically the concept of null. This enum is Option<T>
    // This enum is even in the prelude, you don't need to bring it in
    // specifically. Because it is in the prelude you can use their options
    // without the :: specification. The options are Some and None.
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    // The type of some_number is Option<i32>, the type of some_char is Option<char>
    // Rust can infer these types because we've specified a value of that type into
    // the Some type and Some(T) has type T.
    // For absent_number Rust requires to annotate the overall Option type. So here
    // you have to tell the that the absent_number is type Option<i32>.

    // Why is using this Option<T> better than having null?
    // Because Option<T> and T are different types, the compiler won't let you use
    // an Option<T> value as if it were definitely a valid value.
    // For example: i8 is not the same as Option<i8>.
    // This code would give an error because you are trying to add an Option<i8>
    // with an i8, which are 2 different types, which is not possible.
    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y
    */
    // This means that rust does not know how to add i8 and Option<i8>. When we have
    // an Option<i8> or Option<T> we do have to worry about possibly not having
    // a value and the compiler will make sure that we handle that, before we go any
    // further. Here you have to convert the Option<T> to a T before you can
    // perform operations that imply to T with it. This helps catch one of the most
    // common issues: assuming that something isn't 0 when in fact it is.
    // Everytime that a value has a type that isn't an Option<T>, you can almost
    // safely assume that the value isn't null.
    // How do you get the T out of Option<T>? Use one of the functions in the
    // documentation that suits your needs: https://doc.rust-lang.org/std/option/enum.Option.html


}

// You can define a function that takes the same kind:
fn route(_ip_kind: IpAddr) {}


// Enums give you a way of saying a value is one of a possible set of values.
// In this case you have V4 or V6 as possible IP addresses. These two options
// are the only options right now, so we can enumerate all the possible
// variants. An enum kind can be either one variant or the other, not both at
// the same time, in this case an IP address can be either V4 or V6. They are
// both still IP addresses so they should be both treated as the same type
// when the code is handling things that apply to any IP.
// Right here we define the IPAddrKind enumeration and list all possible kinds
// of IpAddresses, in this case V4 and V6. IpAddrKind is now a custom type.

// Now you do not know what the IpAddresses will be so you can now put (String)
// behind the types to tell that both the variants will have associated String
// values with them.
enum IpAddr {
    V4(String),
    V6(String),
}
// There is another advantage of the enum over the struct: each variant can have
// different types and amount of associated data. Look at this IpAddr2
// (I called it this way to not have to rewrite the first IpAddr). You already
// know that the V4 version cannot have more than 255 in each of the 4 spots
// So you can give it u8's and make it more precise. The V6 does not have that
// option, but it's good that the V4 has, cause now your code is faster.
// You could not do this with structs.

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Wanting to store the Ip addresses enum is so common that the standard library
// already has this covered. The standard library does it like this:
// They make structs of each type and store those in the enum.
// You can put any type of data into an enum, variables, structs, even other enums.
// Standard libraries are most of the time not much more complicated than what
// you make yourselves.

// We can still use our own IpAddr because we didn't get in IpAddr from the library.
// It is currently not in scope.
/*
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/

// Defining enums is much more convenient in some case than naming structs:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// This enum is the same as naming all these structs down here below.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// With structs you cannot have another types of message, you all need to
// have this message. With the enum you can.

// There is one more similarity between the enum and struct. You can have
// You can also define methods for enums, also with the impl.

impl Message {
    fn call(&self) -> self {
        self
    }
}
