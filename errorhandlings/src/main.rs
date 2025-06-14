mod usingpanic;
mod resultenums;
use usingpanic::using_panic;
use resultenums::resultenums;

fn main() {
    println!("Hello, world!");
    using_panic();
    resultenums();
}
