use std::fs::File;
use std::io::ErrorKind;
pub fn resultenums() {
    // it denotes the sucess or failure of an operation
    enum Result<T, E> {
        Ok(T), // Represents a successful result stores generic value
        Err(E),// Represents an error result stores generic error values
    }
    let fls = File::open("hello.txt"); // or // let fls = File::open("hello.txt").unwrap(); // panics if file not found no need to handle error

    let fls: File = match fls {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

if f = File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| panic!("Problem creating the file: {:?}", e))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    }) {
        println!("File opened successfully: {:?}", f);
    }

}