struct User{
    name: String,
    company: String,
    age: u32
}
// The User struct is a simple representation of a user with a name, company, and age.
impl User {
    fn new(name: String, company: String, age: u32) -> User {
        User { name, company, age }
    }
}
// The new function is a constructor for the User struct, allowing us to create a new User instance with the given name, company, and age.
impl User {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_company(&self) -> &String {
        &self.company
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
// The get_name, get_company, and get_age functions are getter methods that return the name, company, and age of the user respectively.
impl User {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_company(&mut self, company: String) {
        self.company = company;
    }
    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}
// The set_name, set_company, and set_age functions are setter methods that allow us to modify the name, company, and age of the user respectively.
impl User {
    fn print_user(&self) {
        println!("User: {} works at {} and is {} years old", self.name, self.company, self.age);
    }
}
fn main() {

//

    let mut u1 = User::new(String::from("Alice1"), String::from("Tech Corp1"), 30);
    let mut u2 = User::new(String::from("Bob2"), String::from("Tech Corp2"), 25);
    // The User struct is created using the new function, which takes a name, company, and age as parameters.
    u1.set_name(String::from("Charlie"));
    u1.set_company(String::from("New Corp"));
    u1.set_age(31);
    u2.set_name(String::from("Dave"));
    u2.set_company(String::from("New Corp"));
    u2.set_age(26);
    u1.print_user();
    u2.print_user();
    // u1.name = String::from("Charlie"); // This will not work because u1 is immutable
    // u1.company = String::from("New Corp"); // This will not work because u1 is immutable

    let u1 = User {
        name: String::from("Alice"),
        company: String::from("Tech Corp"),
        age: 30,
    };
    let mut u2 = User {
        name: String::from("Bob"),
        company: String::from("Tech Corp"),
        age: 25,
    };
    u2.age = 26;
    println!("User: {} works at {} and is {} years old", u2.name, u2.company, u2.age);
     println!("User: {} works at {} and is {} years old", u1.name, u1.company, u1.age);
    // u1.age = 31; // This will not work because u1 is immutable
    // u1.name = String::from("Charlie"); // This will not work because u1 is immutable
    // u1.company = String::from("New Corp"); // This will not work because u1 is immutable
    // u2.name = String::from("Charlie"); // This will work because u2 is mutable
    // u2.company = String::from("New Corp"); // This will work because u2 is mutable
    // u2.age = 31; // This will work because u2 is mutable
    // u2 = u1; // This will not work because u1 is immutable
    // u1 = u2; // This will not work because u1 is immutable
    // u1 = User { name: String::from("Charlie"), company: String::from("New Corp"), age: 31 }; // This will not work because u1 is immutable
    // u2 = User { name: String::from("Charlie"), company: String::from("New Corp"), age: 31 }; // This will work because u2 is mutable
   
}
