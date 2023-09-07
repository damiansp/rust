mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }


    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}
        
        pub fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // Abs path
    //create::front_of_house::hosting::add_to_waitlist();

    // Rel path
    front_of_house::hosting::add_to_waitlist();
}