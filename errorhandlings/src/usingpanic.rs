pub fn using_panic() {
   a();
   panics();
}
#[warn(dead_code)]
pub fn panics() {
    // This will cause a panic
    panic!("This is a panic message"); //it will stop the program execution
    // The following line will not be executed
    println!("This line will not be printed because of the panic above.");
}

fn a(){
    b();
}
fn b() {
    c(0); // This will cause a panic because we are passing 0
}
fn c(num: i32) {
    if num == 0 {
        panic!("num cannot be zero");
    } else {
        println!("num is: {}", num);
    }
}