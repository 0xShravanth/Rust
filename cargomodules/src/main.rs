fn main() {
    println!("Hello, world!");
}

// cargo install cargo-modules
//cargo modules generate tree --lib 
// cargo new -> create a new pacakage -> package stores create crate ( binary crate or lib crate ) -> crate contains modules 
// main.rs is the entry point of the package/ root of the package
// if lib crate is created, then lib.rs is the entry point of the package/ main root of the package

// crate rules:
// 1. crate can contain multiple modules
// 2. module can contain multiple submodules
// 3. module can contain multiple functions

// 4. package can have 0 or 1 lib crate
// 5. package can have 0 or more binary crate bin folder contain binary crate

// creating a lib crate
// cargo new --lib cargomodules

// creating a binary crate
// cargo new --bin cargomodules


// cargo install cargo-modules
    // cargo modules generate tree --lib 
    // cargo new -> create a new pacakage -> package stores create crate ( binary crate or lib crate ) -> crate contains modules 
    // cargo build -> build the package
    // cargo run -> run the package
    // cargo test -> test the package
    // cargo doc -> generate documentation for the package
    // cargo fmt -> format the code
    // cargo clippy -> lint the code
    // cargo update -> update the dependencies
