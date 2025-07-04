fn main() {
/*
Generics allow you to write flexible,
reusable code without sacrificing type safety.
Rather than writing separate functions or types for each data type,
you write a single definition that works with many types
*/
// Example of a generic function
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 10, y: 20 };
let float_point = Point { x: 1.0, y: 2.0 };

print_value(42);              // works with i32
print_value("hello");         // works with &str
print_value(vec![1, 2, 3]);   // works with Vec<i32>
add(5, 10);               // works with i32
add(1.5, 2.5);         // works with f64

}
fn print_value<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
/*

*/
