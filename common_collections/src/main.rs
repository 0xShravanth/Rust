// collectoion are allocated in the heap
//note for arrays size is know at compile time
// for vec size is know at runtime

 // Declares the module
mod vectoraccess;
mod stringcollection;
mod hashmapcollections;


// Import the function
use crate::vectoraccess::vectoraccess::print_vector_access; // absolute path
use stringcollection::stringcollection::print_string_collection; // relative path
use stringcollection::stringcollection::print_string; 
use stringcollection::stringcollection::indexing_into_string; 
use stringcollection::stringcollection::grapheme_clusters;
use hashmapcollections::print_hashmap_collections; // relative path

fn main() {
     println!("\n");  // Print a new line for better readability
    //calling the function from the vectoraccess module
    print_vector_access();

    print_string_collection();

    print_string();  

    indexing_into_string();

    grapheme_clusters();

    print_hashmap_collections();

}
