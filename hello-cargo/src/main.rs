mod car_exercise;
mod hashmap;
mod loops;
mod person_exercises;

// -------------------- Enums ----------------------- //
#[derive(Debug)]
enum WebEvent {
    Weber(bool),
}

fn handle_web_event(event: WebEvent) {
    match event {
        WebEvent::Weber(value) => {
            if value {
                println!("Weber is active");
            } else {
                println!("Weber is not active");
            }
        }
    }
}

fn main() {
    // ------------------ Variables ------------------- //

    let a_number: u8 = 1;

    let a_word = "One";

    let shadow_num: u8 = 42;

    let shadow_num = shadow_num + 1;

    let shadow_num = shadow_num * 3;

    let mut name = "Ale";

    println!("pile Name: {name}");

    name = "Ale Weber";

    let _word_a: char = 'A';
    let _word_w: char = 'W';

    let bye = "Goodbye Horses";

    let number = 12;

    // --------- Tuple --------- //

    let tuple_a = ('A', 15, true, "Hello, it's me...");

    // --------- struct ---------- //

    struct Student {
        name: String,
        birth_date: String,
        dev: String,
        age: u8,
    }

    let ale: Student = Student {
        name: String::from("Ale"),
        birth_date: String::from("16-10-1998"),
        dev: String::from("Backend"),
        age: 25,
    };

    struct Grades(char, char, char);

    let mark_1 = Grades('A', 'B', 'C');

    // ----------------- Functions -------------------- //

    println!("Hello, world!");

    println!("Hello {}, General {}", "There", "Kenobi!");

    println!("the value of shadow_num is: {}", shadow_num);

    println!("de name is: {name}");

    println!("{} is a number, {} is a word", a_number, a_word);

    println!("initials: {} y {}", _word_a, _word_w);

    println!("This is a Tuple: the letter {}, the number {}, the boolean: {}, and the part of the song: {}", tuple_a.0, tuple_a.1, tuple_a.2, tuple_a.3);

    println!(
        "mark 1: {}, mark2: {}, mark3: {} ",
        mark_1.0, mark_1.1, mark_1.2
    );

    println!(
        "Name: {}, birth date: {}, dev: {}, age: {}",
        ale.name, ale.birth_date, ale.dev, ale.age
    );

    let last_name = WebEvent::Weber(true);

    println!("This is an Enum: {:?}", last_name);

    fn goodbye(message: &str) {
        println!("\n{}", message)
    }

    fn divide_by_5(num: u32) -> u32 {
        num / 5
    }

    handle_web_event(last_name);

    divide_by_5(number);

    goodbye(bye);

    println!("{number} divided by 5 is {}", divide_by_5(number));

    person_exercises::person_exercise();

    car_exercise::exercise();

    if 1 == 2 {
        println!("True, the numbers are equal.")
    } else {
        println!("False, the numbers are not equal")
    };

    let formal = true;
    let greeting = if formal { "Good day to you" } else { "Hey!" };

    println!("{greeting}");

    hashmap::hash_map();

    loops::loops();

    todo!("***** Finish to learn Rust! *****");
}
