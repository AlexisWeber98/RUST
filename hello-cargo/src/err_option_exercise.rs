// EJERCICIO: Uso del tipo Option para tratar la ausencia

#[derive(Debug)]
struct Person {
    first_name: String,
    second_name: Option<String>,
    last_name: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first_name);
    full_name.push_str(" ");

    if let Some(second_name) = &person.second_name {
        full_name.push_str(&second_name);
        full_name.push_str(" ")
    }

    full_name.push_str(" ");
    full_name.push_str(&person.last_name);

    return full_name;
}

pub fn full_name() { 
    let peter = Person {
        first_name: String::from("Peter"),
        second_name: Some(String::from("John")),
        last_name: String::from("McKenzie"),
    };
    
    assert_eq!(build_full_name(&peter), "Peter John Mackenzie");
    println!("person 1: {peter:?}");

    let alice = Person {
        first_name: String::from("Alice"),
        second_name: None,
        last_name: String::from("Smith"),
    };

    assert_eq!(build_full_name(&alice), "Alice Smith");
    println!("person 2: {alice:?}");

    let robert = Person {
        first_name: String::from("Robert"),
        second_name: Some(String::from("Bastian")),
        last_name: String::from("Zem"),
    };

    assert_eq!(build_full_name(&robert), "Robert Bastian Zem");
    println!("person 3: {robert:?}");
}
