You can achieve this by creating a separate module in `vectoraccess.rs` and then using that module in your `main.rs`. Here’s how you can structure your code:

### 1️⃣ **Create a new file:** `src/vectoraccess.rs`

```rust
pub fn print_vector() {
    let vec = vec![10, 20, 30, 40, 50];
    println!("Vector elements: {:?}", vec);
}
```

This defines a function `print_vector()` inside `vectoraccess.rs` and makes it public (`pub`) so it can be accessed from `main.rs`.

### 2️⃣ **Modify your `src/main.rs`**

```rust
mod vectoraccess; // Declare the module

fn main() {
    vectoraccess::print_vector(); // Call the function from the module
}
```

Here, `mod vectoraccess;` tells Rust to include `vectoraccess.rs` as a module, and then you call the function using `vectoraccess::print_vector();`.

### 3️⃣ **Run your project**

```sh
cargo run
```

This will execute `main.rs` and call the function from `vectoraccess.rs`.

🚀 Now your `vectoraccess.rs` file is being used in `main.rs`! This keeps your code **organized** and helps maintainability.

---

Yes! Another way is to use Rust's **`mod` and `use` keywords** to structure your modules more explicitly.

### 1️⃣ **Create `src/vectoraccess.rs`**

Define a module with a function:

```rust
pub fn fetch_vector() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}
```

### 2️⃣ **Modify `src/main.rs` to use `vectoraccess`**

```rust
mod vectoraccess; // Include the module

use crate::vectoraccess::fetch_vector; // Import the function explicitly

fn main() {
    let vec = fetch_vector();
    println!("Vector from module: {:?}", vec);
}
```

### 🚀 **How This Works**

- `mod vectoraccess;` brings in the module.
- `use crate::vectoraccess::fetch_vector;` imports the function explicitly, making calls **cleaner**.
- Now, you can call `fetch_vector()` **directly** in `main.rs` without `vectoraccess::` prefix.
