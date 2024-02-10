use std::fmt:Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else { 
            println!("The largest number is y = {}", self.y);
        }
    }
}

// Use the default implementation of the trait
impl Summary for NewsArticle {}

//impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl trait - trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// ^ above is syntax sugar for trait bound syntax
//pub fn notify<T: Summary>(item: &T) {
//    println!("Breaking news! {}", item.summarize());
//}
//
// sometimes using the trait bound syntax
// for example, if we have multiple parameters of the same type
// 
// impl trait
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//
// trait bound
//pub fn notify<T: Summary>(item1: &T, item2: &T) {

// We can specify multiple trait bounds with the '+' syntax
//pub fn notify(item: &(impl Summary + Display)) {
// 
// This also works on trait bounds
//pub fn notify<T: Summary + Display>(item: &T) {
//
//
// Signature 'where' clause
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//
// we can use the where clause like this
// fn some_function<T, U>>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {


// we can specify the return type as
// any type implements the Summary Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("penguinslayer69"),
        content: String::from("lolol git rekt"),
        reply: false,
        tetweet: false,
    }
}


// This is not allowed - you can only return a single type from a trait return
//fn returns_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("I liek carrots"),
//            location: String::from("Somewhere, USA"),
//            author: String::from("Ferris"),
//            content: String::from("Do u liek carrotz to02??!?"),
//        }
//    } else {
//        Tweet {
//            username: String::from("elonmusk"),
//            content: String::from("Mars has officially been defunded and will be destroyed"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}


