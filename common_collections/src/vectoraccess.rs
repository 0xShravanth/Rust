pub mod vectoraccess {

    pub fn print_vector_access() {
        // collectoion are allocated in the heap
        //note for arrays size is know at compile time
        // for vec size is know at runtime
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let mut b: Vec<i32> = Vec::new(); // calling vec::new() creates a new vector
        b.push(1);
        b.push(2);
        b.push(3);
        let v2 = vec![1, 2, 3];

        // accessing elements inside a vector'
        // note accesing element using this method is unsafe can cause panic if the index is out of bounds
        // so we should use get() method to access elements safely
        let acc_element = &a[2]; // accessing the third element (index 2)
        println!("The third element of a is: {}", acc_element);
        println!("The vector v2 is: {:?}", v2[1]);

        // safer way to access elements using get() method 
        if let Some(value) = b.get(1) {
            println!("The second element of b is: {} using if let", value);
        } else {
            println!("No second element in b to access");
        }
        
        // or use this way
        match b.get(1) {
            Some(value) => println!("The second element of b is: {} using match", value),
            None => println!("No second element in b to match"),
        }
        println!("");
    
        // using enumerate to get index and value
        for (index, value) in b.iter().enumerate() { // iterating with index
            // iter() returns an iterator over references to the elements
            println!("Element at index {} is: {}", index, value);
        }
        println!("");

        // using iterators to access elements
        for i in & mut b { // iterating over references to elements in b
            *i = *i  + 1 ; // modifying the element in place , *i dereferences the value
            // *i += 1; // another way to modify the element in place same as above
            // note that we are using &mut b to get mutable references to the elements
            println!("Element in b is: {}", i);

        }
        println!("");

        //using enums 
        let row = vec![
            SpreadsheetCell::Int(10),
            SpreadsheetCell::Float(3.14),
            SpreadsheetCell::Text(String::from("Hello")),
        ];
        match &row[1] {
            SpreadsheetCell::Int(value) => println!("Integer value: {}", value),
            _=> println!("Not an integer"),
        }
         println!("\n");

    }
    #[allow(dead_code)] // to avoid unused enum warning
    #[derive(Debug)] // derive Debug trait to print the enumerate
    pub enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

}