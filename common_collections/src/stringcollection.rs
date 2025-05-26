pub mod stringcollection {
    // strings are stored as a collection of UTF-8 encoded bytes
    // as string is utf-8 encode we can write in many languages
    // String is a growable, heap-allocated data structure
    
    use std::collections::HashMap; // Importing HashMap from the standard library
    use unicode_segmentation::UnicodeSegmentation; // Import the trait



    pub fn print_string(){
        // Creating new String
        let s1 = String::new(); // Creates an empty String
        let s2 = "string literal"; // String literal
        let s3 = s2.to_string(); // Converts a string literal to a String
        let s4 = String::from("creating string from string literal"); // Creates a String from a string literal
        println!(" s1: {},\n s2: {},\n s3: {},\n s4: {}", s1, s2, s3, s4);
        println!("");

        let mut s = String::from("food "); // Creating a mutable String
        s.push_str("is good"); // Appending a string slice to the String
        s.push('!'); // Appending a character to the String
        println!("s after push_str and push: {}", s);
        // can also append  using + operator
        println!("");

        let sappend = String::from(" and healthy");

        
        let s6 = format!("{} {}", s, sappend); // Using format! macro to concatenate strings without moving ownership
        // format! does not take ownership of the strings, so both s and sappend can still be used
        println!("s6 after format!: {}", s6);
        println!("");

        // Concatenating two Strings using the + operator
        // Note: The + operator takes ownership of the first String, so it cannot be used after this line
        let s5 = s + &sappend; // Concatenating two Strings using the + operator
        // Note: The `s` variable is moved here, so it can no longer be used after this line
        println!("s5 after concatenation: {}", s5);
        println!("");

        // let s6 = format!("{} {}", s, sappend); // Using format! macro to concatenate strings without moving ownership
        // // format! does not take ownership of the strings, so both s and sappend can still be used
        // println!("s6 after format!: {}", s6);
        // println!("");
    }

    pub fn indexing_into_string() {
        // Indexing into a String
        let s = String::from("indexing into string");
        println!("String is: {}", &s); 
        let first_char = s.chars().nth(0).unwrap(); // Getting the first character using chars() iterator
        println!("First character of the string is: {}", first_char);
        
        // Note: Direct indexing like s[0] is not allowed in Rust for Strings because it can panic if the index is out of bounds
        // Instead, we use methods like chars() or bytes() to safely access characters or bytes.


        // Accessing bytes of a String
        let bytes = s.as_bytes(); // Converting the String to a byte slice
        println!("Bytes of the string: {:?}", bytes);
        let bytes = s.as_bytes()[1]; 
        println!("Second byte of the string: {}", &bytes);
        println!(" convering byte : {} as a char :{}", &s.as_bytes()[1] , s.as_bytes()[1] as char); // Converting the byte to a char for display
        println!("");

    }


    pub fn grapheme_clusters() {

        //cargo add unicode-segmentation 
        // Grapheme clusters in Rust
        let s = String::from("नमस्ते"); // A string with multiple grapheme clusters
        println!("String is: {}", &s);
        
        // Iterating over grapheme clusters
        for c in s.chars() {
            println!("Grapheme cluster: {}", c);
        }
        println!("");
        // Note: The `chars()` method returns an iterator over the Unicode scalar values (grapheme clusters) in the string.

        // Using unicode-segmentation crate to get grapheme clusters
        let text = "हेलो, Rust! 🌍"; // Unicode text
        let chars: Vec<&str> = text.graphemes(true).collect();

        println!("Characters: {:?}", chars);
        println!("Number of grapheme clusters: {}", chars.len());
        println!("");

        // Displaying each grapheme cluster
        for (i, grapheme) in chars.iter().enumerate() {
            println!("Grapheme {}: {}", i + 1, grapheme);
        }
        println!("");
        // Note: The `graphemes` method returns an iterator over the grapheme clusters, which can be collected into a vector or iterated directly.
        for g in "हेलो, Rust! 🌍".graphemes(true){
            println!("Grapheme: {}", g);
        }

        println!("");
    }

// AG
    pub fn print_string_collection() {
        // Creating a new HashMap
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Accessing values in the HashMap
        let blue_score = scores.get("Blue");
        match blue_score {
            Some(score) => println!("Blue score: {}", score),
            None => println!("No score for Blue"),
        }

        // Iterating over the HashMap
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // Using entry API to insert or update values
        scores.entry(String::from("Blue")).or_insert(30); // This will not change the value for "Blue" since it already exists
        scores.entry(String::from("Green")).or_insert(20);

        // Displaying updated scores
        println!("Updated scores:");
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
        println!("\n"); 
    }
}