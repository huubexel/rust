fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // let's match these 2 plus_one uses
    // For the first one, it goes to the function with the match
    // It sees None, and doesn't match
    // Some(i) => Some(i + 1), the some i matches here, so it returns six.

    // Now for the second call: None matches None, there's no value to add,
    // so the match is found, so no other arms will be compared and
    // the None is returned.

    // Combining enums and matches is used a lot, like a lot a lot.
    // Remember this one.

    // Matches are exhaustive.
    // This means that the arms of the match must cover all possibilities.
    // Say you would have this code:
    /*
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
     */
    // This code would obviously not compile because all possibilities are not covered.
    // The possibility of having nothing, it left out. 'None' isn't covered.
    // This goes for every enum, every scenario of the enum must be covered.

    // You have a game where when you roll 3 or 7 something happens, if you roll anything
    // else you move the amount you just rolled. This is basically all other options.
    // Say you want to match all those other options.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // That pattern is always the last pattern to match in the match and for this
    // I used other as a variable name, but you can use something else.
    // This works because the last pattern matches all other patterns that aren't 3 or 7.

    // Rust also has a pattern for if we do want a catch-all pattern, but we do not want to
    // use that value. The _ pattern. This tells rust we are not going to use that value.
    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(),
    }
    // Nice, we haven't forgotten anything.
    // This will work just fine.

    // Now say, for that same game, the rules have changed since the last time you played it.
    // This time if you do not roll a 3 or 7, nothing will happen and your turn will pass.
    // You can express this by putting a unit value in the last arm, look at this code:
    let dice_roll3 = 9;
    match dice_roll3 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Than there is one more thing about enums and pattern matching
    // That is the concise Control flow with if let
    // The if let syntax lets you match one pattern while ignoring the rest.
    // This looks like this:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // This will print the line with the variable and will otherwise ignore
    // it when none is given.
    // However to everytime type in _ => () is annoying, so Rust has a shorthand for this.
    let config_max2 = Some(3u8);
    if let Some(max2) = config_max2 {
        println!("The maximum is configured to be {}", max2);
    }
    // This is syntactic sugar for if you have a match where you only have
    // to match one pattern and can ignore all other patterns.

    // But say you now have this code:
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // And you really don't want to use a match here, than you can use if let else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // Which is a bit prettier in this situation.
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn re_roll() {}

enum _Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


// Match is a control flow which allows you to compare a value against a series
// of patterns and execute code based on which pattern it matches.
// Match is like a coin-sorting machine. Coins slide down a track and come in the
// right spot in the end. Look at this piece of code where the example is explained.
fn _value_in_cents(coin: _Coin) -> u8 {
    match coin {
        // the numbers after the => get returned.
        _Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        _Coin::Nickel => 5,
        _Coin::Dime => 10,
        _Coin::Quarter => 25,
    }
}
// This match is basically an if, however there is one exception:
// with if the expression needs to return a boolean expression, with
// match, this is not the case. This returns the type Coin.

// The values of each arm gets compared to the value in order, from top to bottom.
// You can have as much arms as needed.

// If the match arm is short, you don't have to use brackets, but with multiple lines
// it is good practice to do so. Like with the penny above here.

// The match arms can bind to the parts of the values that match the pattern.
// For example look at this here below, in America the quarter has a different print
// for each state. If you now throw the coins in the sorting machine, we can also
// call out the state that is on the quarter.

enum _UsState {
    Alabama,
    Alaska,
    // etc.
}

enum _Coin2 {
    Penny,
    Nickel,
    Dime,
    //Quarter(UsState),
}

// Matching with Option<T>, some or none

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// Here if there isn't a value inside, the function should return the none value
// and not attempt any operations further down the line.