mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Adding to waitlist");
        }
        pub fn seat_at_table(){
            println!("Seating at table");
        }
    }
    mod serving{
        fn take_order(){
            println!("Taking order");
        }
        fn serve_order(){
            println!("Serving order");
        }
        fn take_payment(){
            println!("Taking payment");
        }
    }
}

pub fn eat_at_restaurant(){
    //creating absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //creating relative path
    front_of_house::hosting::seat_at_table();
}



// cargo modules structure --lib
// cargo modules generate tree --bin