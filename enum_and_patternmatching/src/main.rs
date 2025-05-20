#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum IpAddrs{
    V4(u8,u8,u8,u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// or we can declare seprate structs for each message
struct QuitMessage; //unit struct

struct MoveMessage {
    x: i32,
    y: i32,
}
// imploementing methods for enum of message
impl Message {
    fn some_function(){
        println!("Hello from some_function");
    }
}

struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
struct IpAddr{
    kind:IpAddrKind,
    address: String,
}
// 
fn main() {
    let localhost = IpAddrs::V4(127,0,0,1);
    let localhost2 = IpAddrKind::V4(String::from("127,0,0,1"));
   let four = IpAddrKind::V4(String::from("192.168.0.1")); // Correct
let six = IpAddrKind::V6(String::from("::1"));          // Correct

   println!("{:?}", four);
    println!("{:?}", six);
    println!("{:?}", localhost);
    println!("{:?}", localhost2);

  
}
fn route(ip_Kind: IpAddrKind )  {}