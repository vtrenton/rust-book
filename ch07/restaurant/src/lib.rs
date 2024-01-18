// package > crates > modules

use std::fmt::Result;
// we can use the 'as' keyword to create an alias
use std::io::Result as IoResult;
// if I want to import multiple items in the same crate
// I can use the following syntax
// this will pull both cmp::Ordering and io from the std package
use std::{cmp::Ordering, io};
// we can also use nested paths
use std::io::{self, Write};
// globs '*' are also supported to pull everything from a module into scope
use std::collections::*;

fn function1() -> Result {
    // do stuff
}

fn function2() -> IoResult {
    // do other stuff
}

mod front_of_house;
//mod front_of_house {
//    pub mod hosting {
//        pub fn add_to_waitlist() {}
//
//        fn seat_at_table() {}
//    }
//
//    mod serving {
//        fn take_order() {}
//
//        fn serve_order() {}
//
//        fn take_payment() {}
//    }
//}

mod back_of_house {
    // with structs every field is private
    // even if the struct is private
    pub struct Breakfast {
        // fields need to be declared as private
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Breakfast needs a public acocaited function
        // within the scope of the Breakfast Struct
        // because seasonal_fruit is private
        // it cannot be constructed outside of the module scope
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // Since the enum is public
    // so are all the varients
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        //cook_order();
        super::deliver_order()
    }
}

// We can reexport this with the pub use method
// now external modules can reference this import with
// resturant::front_of_house::hosting
pub use crate::front_of_house::hosting; 

pub fn eat_at_resturant() {
    // Absolute Path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative Pth
    //front_of_house::hosting::add_to_waitlist();

    // since we narrowed the scope with the use above
    // we no longer need to specify front_of_house
    // This is a lot like a symlink
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // both varients can be accessed since the enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}
