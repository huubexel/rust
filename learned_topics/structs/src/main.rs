fn main() {
    // Structs hold multiple related values (just like tuples) which can be of different types,
    // unlike tuples you name each piece of data so it's clear what those values mean. Because of
    // this structs are more flexible, you do not have to rely on the order the data is in.

    // You make an instance of a struct by specifying the values you want for the fields:
    let mut user1 = User {
        _active: true,
        _sign_in_count: 1,
        email: String::from("someone@example.com"),
        username: String::from("some_username_123"),
    };
    // As you can see, structs have key value pairs. As you can see, you do not have to
    // fill in the fields, the same order.
    // To get a value from the struct you use the dot notation, f.e. user1.email
    // If this email is mutable, you can change it.
    user1.email = String::from("another_email@example.com");

    // Notice that the hole instance has to be mutable! You cannot make some parts mutable and
    // some parts immutable, the hole instance is mutable or not.

    // You can even make a function like build_user to build a User quite simply.
    // You should name the parameters the same way as the key in the struct.
    // However if struct is big, this having names somewhere twice is a bit tedious.
    // There is a shorthand version for this.
    // Look at build_user_short, you do not have to use the :email and :username,
    // but you can just use the name and that's it.

    // You can use the values from another instance to make the next instance.
    // This is called struct update syntax. Take a look at this code.
    let user2 = build_user_short(
        String::from("someone@example.com"),
        user1.username
    );

    // The only thing we want to change is the email, so you can use the shorthand struct update
    // syntax to do just that:
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // We can now no longer use the user1 or user2, because of the = sign, the values
    // of one of these moved along the way.
    //let name: String = user2.username;
    // --> this will not work, because the values of user2 (except email) moved to user3
    // Now the rest of these values would have worked because active and sign_in_count are
    // of the trait Copy and the email is from user3 itself, but the username can't be used.

    // Rust also supports structs that look similar to tuples without keys,
    // These are called tuple structs. These are useful when you want to give the tuple a name
    // to make it stand out from other tuples and when naming the values would be redundant:
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // You can even make structs that do not have fields --> unit-like structs.
    // These behave similar to the unit () and are useful when you need to implement a trait
    // on something, here is an example:
    let _subject = AlwaysEqual; // can also be useful for testing later on.

    // It is possible for structs to store references to data however it does require lifetimes,
    // This will be covered in chapter 10.
}


// To define a struct you use the keyword struct and the name of the struct after it.
// The names and types of the data inside are called fields. F.e.:
struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        _active: true,
        _sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
