fn main(){
    println!("Hello, bin module!");
    eat_at_restaurant()
    
}
fn serve_order(){
        println!("Serving order");
    }

mod back_of_house{
    pub fn fix_incorrect_order(){
        cook_order();
        super::serve_order(); // calling the function from the parent module use super
        // serve_order(); // this will not work as the function is private to the module
        take_payment();
    }

    fn cook_order(){
        println!("Cooking order");
    }
    
    fn take_payment(){
        println!("Taking payment");
    }
}

 mod front_of_house {
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

    back_of_house::fix_incorrect_order();
}
// cargo modules structure --lib
// cargo modules structure --bin

// PS D:\Rust\cargomodules> cargo modules structure --bin binary_crate

// crate binary_crate
// ├── mod front_of_house: pub(crate)
// │   ├── mod hosting: pub
// │   │   ├── fn add_to_waitlist: pub
// │   │   └── fn seat_at_table: pub
// │   └── mod serving: pub(self)
// │       ├── fn serve_order: pub(self)
// │       ├── fn take_order: pub(self)
// │       └── fn take_payment: pub(self)
// └── fn main: pub(crate)