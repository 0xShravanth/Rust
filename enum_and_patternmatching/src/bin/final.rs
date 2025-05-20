//option enum
#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => { 
            println!("Lucky penny!{} ", 1);
            1
            },
        Coin::Nickel => {
            println!("Lucky nickel!{} ", 5);
            5
            },
        Coin::Dime => {
            println!("Lucky dime!{} ", 10);
            10
            },
        Coin::Quarter(state) =>   {
            println!("Lucky quarter from {:?}!{} ", state, 25);
            25
            },
            // handling all other cases
            // _ => {
            //     println!("for all remaining case ");
            //     0
            // }
    }
}

fn main() {
    // calling the enum functions from above
    let coin = Coin::Quarter(UsStates::Alabama);
    value_in_cents(coin);

    let my_state = UsStates::Alaska; // Now Alaska is used
println!("{:?}", my_state);

let my_coin = Coin::Penny; // Now Penny is used
println!("Coin value: {:?}", my_coin);

    let _unused_coin = Coin::Dime;

    // Option enum is used to represent a value that can be either Some(value) or None
    #[derive(Debug)]
    enum CustomOption<T> {
        Some(T),
        None,
    }

    let x = Some(5);
    let y: Option<i32> = None;  // Explicit type for Option<i32>

    match x {
        Some(value) => println!("Value is: {}", value),
        None => println!("No value"),
    }

    match y {
        Some(value) => println!("Value is: {}", value),
        None => println!("No value"),
    }
 // using prifix _ to avoid warning
    let _some_number = Some(10);
    let _no_number: Option<i32> = None; // Uses Rust’s built-in Option type
    let _no_number2: CustomOption<i32> = CustomOption::None; 

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
    // using the Option enum

    //using if let
    let i = 5;
    // using if let to check if the value is Some
    if let Some(i) = five { // or directly use value inside some ie some(5)
        println!("The value is: {}", i);
    } else {
        println!("No value");
    }
}
//
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
        //or we can use the shorthand
        // _ => None
    }
}
