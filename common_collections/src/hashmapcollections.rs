
pub fn print_hashmap_collections() {
    use std::collections::HashMap; // bringing HashMap into scope



    // Creating a new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in the HashMap
    let team_name = "Blue";
    let blue_score = scores.get(&team_name[..]); // Using a slice to get the string
    //let blue_score = scores.get(team_name.as_str());// Using as_str() to get a string slice
    //let blue_score = scores.get(&team_name[..]);
    match blue_score {
        Some(score) => println!("Blue score: {}", score),
        None => println!("No score for Blue"),
    }

    // Iterating over the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Using entry API to insert or update values
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(20); // This won't change the existing value

    println!("Updated scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("Updating scores:");
    
        // Insert values
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 15);

    // Overwrite value
    scores.insert("Blue".to_string(), 20);

    // Modify existing value
    scores.entry("Blue".to_string()).and_modify(|score| *score += 5);

    // Insert if missing
    scores.entry("Green".to_string()).or_insert(25);

    // Displaying updated scores
     println!("Final HashMap: {:?}", scores);

    println!("\n"); // Print a new line for better readability

    for word in "hello world wonderful world".split_whitespace() {
        let count = scores.entry(word.to_string()).or_insert(0);//
        *count += 1; // Increment the count for each word
    }
    
    for (key, value) in &scores {
        println!("final output unsorted {}: {}", key, value);
// 1️⃣ "wonderful: 1" appears first because it was added last (during word counting).
// 2️⃣ "Red: 15" comes next because it was inserted early.
// 3️⃣ "hello: 1", "Yellow: 50", "Blue: 25", "Green: 30" follow in their respective inserted positions.
// 4️⃣ "world: 2" appears at the end because it was upda ted
// However, this order is not strictly guaranteed—it might change across runs.


    }
    println!(""); 

    let mut sorted_scores: Vec<(&String, &i32)> = scores.iter().collect();
    sorted_scores.sort_by(|a, b| a.0.cmp(b.0)); // Sort by key (alphabetically)

    //  Uses .iter().collect() to convert the HashMap into a sortable vector.
    //  Sorts alphabetically based on the key (a.0.cmp(b.0)).

println!("Sorted HashMap:");
for (key, value) in sorted_scores {
    println!("{}: {}", key, value);
}
}



// for (key, value) in &mut scores {
    //     // updating existing key 
    //     scores.insert("Green".to_string(), 30); // Overwrites existing "Green" score
    //     //Use .entry() to conditionally modify a value:
    //     scores.entry("Blue".to_string()).and_modify(|score| *score += 5);
    //     println!("{}: {}", key, value);
    // }
    