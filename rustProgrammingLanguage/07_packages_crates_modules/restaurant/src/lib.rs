use std::ftm::Result;
use std::io::Result as IoResult;
use std::io::{self, Write}; // self is io


mod front_of_house;

pub use crate::front_of_house::{hosting, serving};


mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_botched_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


pub fn eat_at_restaurant() {
    // Abs path
    //create::front_of_house::hosting::add_to_waitlist();

    // Rel path
    //front_of_house::hosting::add_to_waitlist();
    // After use import
    hosting::add_to_waitlist();
    // Order breakfast in summer w Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about toast choice
    meal.toast = String::from("Wheat");
    println!("Actually I'd like {} toast, please.", meal.toast);

    //meal.seasonal_fruit = String::from("blueberries");  // err: private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


fn deliver_order() {}