use std::collections::HashMap;

pub fn hash_map() {

    let mut reviews: HashMap<String,String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate"));
    reviews.insert(String::from("Cooking with Ale"),String::from("Sweet recipes"));
    reviews.insert(String::from("Learning Rust"),String::from("Great beginner's book"));

    let book: &str = "Learning Rust";
    println!("\nReview for '{}': {:?}", book, reviews.get(book));


    //remove book review 

    let obsolete : &str = "Ancient Roman History";
    println!("\n'{}' Removed", obsolete );

    reviews.remove(obsolete);

    //  confirm book review removed

    println!("\nReview for '{book:?}': {:?}", reviews.get(obsolete) );


}