fn main() {
    // Traits are what are in other languages known as interfaces. They do have some differences.

    // A trait is a types behavior. It exists out of methods we can call on that type. Different
    // types share the same behavior if we can call the same methods on all of those types.
    // Trait definitions are a way to group method signatures together to define a set of behaviors
    // necessary to accomplish some purpose.

    // Example:
    // Say you have various kinds of structs that hold various kinds of text,
    // f.e. 'NewsArticle' struct that holds a news story filed in a particular location.
    // f.e. 'Tweet' that can have at most 280 characters along with metadata

    // Now we want to make a library crate named 'aggregator', that can display
    // summaries of data that might be stored in a 'NewsArticle' or 'Tweet' instance.
    // To do this, we need a summary from each type, this will be done by
    // calling a summarize method on an instance.

    // This code is going to be expanded that's why I'll explain it here.
    /*
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    */
    // This piece of code is a starting point of how to do this:
    // Here you declare a trait using the trait keyword and than the trait name --> Summary
    // The keyword pub is also added which means that it is public so that crates depending
    // on this crate can make use of this trait too. In the curly brackets
    // the method signatures are declared that describe the behaviors of the types that
    // implement this trait. In this case is that fn summarize(&self) -> String;
    // As you can see the method is not opened and defined here, every type that has
    // the Summary trait must have the method summarize defined with this signature.
    // If the type has the Summary trait and there is not a method defined with this
    // signature, a compiler error will follow.


    // Implementing a trait on a type:
    // In the code below the trait is implemented on Tweet and NewsArticle
    // you do this with the: impl NameOfTrait for StructName
    // within the impl goes the method signature that the trait needs you to define.
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}
