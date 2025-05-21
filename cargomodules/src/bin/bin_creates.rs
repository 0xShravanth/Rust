 mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
         pub seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
// use crate::front_of_house::hosting; used to import the hosting module
pub use crate::back_of_house::Breakfast;
// or can be inported like this below
//use self::back_of_house::Breakfast; // used to import the hosting module

// use pub for exporting or external file use

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer(String::from("Rye"));
  // using imported modules
  let  meals = Breakfast::summer(String::from("Rye"));
  meal.toast = String::from("Wheat");
  
  println!("I'd like {} toast please", meal.toast);
  println!("I'd like {} fruit please", meal.seasonal_fruit);
  println!("I'd like {} toast please", meals.toast);
}

fn main() {
    eat_at_restaurant();
    // let mut meal = back_of_house::Breakfast::summer(String::from("Rye"));
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} toast please", meal.toast);
    // println!("I'd like {} fruit please", meal.seasonal_fruit);
}