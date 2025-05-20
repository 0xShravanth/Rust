struct User {
    name: String,
    email: String,
    sign_in_count: u32,
    active: bool
}
impl User {
    // creating a constructor
    fn display(&self) {
        println!("User: {} works at {} and is {} years old", self.name, self.email, self.sign_in_count);
    }
}
// associated functions
impl User {
    // creating a constructor
    fn new_user(name: String, email: String) -> User {
        User {
            name,
            email,
            sign_in_count: 1,
            active: true
        }
    }
}

// creating a struct

// creating a user constructor
fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1
    }
}

fn main(){
    let mut u1 = User {
        name: String::from("Alice"),
        email: String::from("abc"),
        sign_in_count: 1,
        active: true,
    };
    let  u2 = User {
        name: String::from("Bob"),
        email: String::from("xyz"),
        sign_in_count: 1,
        active: true,
    };
    u1.name = String::from("Charlie");

// using struct update syntax ie struct constructor
// using the function to create a user
    let user1 = build_user(
        String::from("xyz"),  
        String::from("Bob")
        );

// using struct update syntax ie struct constructor
// using enum to create a user
    let user2 = User {
        email: String::from("xyz1"),
        name: String::from("Bob1"),
        ..user1
    };


    println!("User: {} works at {} and is {} years old. current status : {}", u1.name, u1.email, u1.sign_in_count , u1.active);
    println!("User: {} works at {} and is {} years old", user1.name, user1.email, user1.sign_in_count);
    println!("User: {} works at {} and is {} years old", user2.name, user2.email, user2.sign_in_count);
    println!("User: {} works at {} and is {} years old", u2.name, u2.email, u2.sign_in_count);
    // u1.name = String::from("Charlie"); // This will not work because u1 is immutable
    // u1.email = String::from("abc"); // This will not work because u1 is immutable


    //creating a tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 1, 0);
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    user1.display();
    user2.display();

// creating a user using associated functions
    let user3 = User::new_user(String::from("xyz"), String::from("Bob new_user"));
    println!("User: {} works at {} and is {} years old", user3.name, user3.email, user3.sign_in_count);
}
