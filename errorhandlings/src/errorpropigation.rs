use std::fs::{self, File};
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),    
    // };
    // // or using the ? operator

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    //  let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(_) => Ok(s),
    // Err(e) => Err(e),


    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
1