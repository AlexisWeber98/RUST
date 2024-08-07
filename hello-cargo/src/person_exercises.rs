pub fn person_exercise() {
    // -------------------------  class --------------//
    #[derive(Debug)]
    struct Person {
        //se crea una estructura (interfaz)
        name: String,
        age: i32,
        dni: i32,
    }

    impl Person {
        fn new(name: String, age: i32, dni: i32) -> Person {
            // se crea una implementación con new() (Class / obj)
            Person { name, age, dni }
        }
    }

    impl ToString for Person {
        fn to_string(&self) -> String {
            format!(
                "Person - Nombre {}, age: {}, dni: {}",
                self.name, self.age, self.dni
            )
        }
    }

    let pepe: Person = Person::new("Pepe".to_string(), 22, 42566832);

    println!("Pepe es: {pepe:#?}"); // ésta sintaxis permite  imprimir la estructura de manera mas legible, parecida e JSON en la consola

    trait MostrarEnConsola {
        fn mostrar_en_consola(&self);
    }

    impl MostrarEnConsola for Person {
        fn mostrar_en_consola(&self) {
            println!("jelooooooow  -> {:?}", self)
        }
    }

    pepe.mostrar_en_consola();

    // ---------------------- array ----------------------//
    let days_array = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    let bytes = [0; 5];

    // Los arrays son inmutables, los vectores fungen como los arrays de JavaScript //

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let mut index_vec = vec![5, 16, 17];

    println!("first day of array: {}", days_array[0]);
    println!("Zeroes: {bytes:?}");
    println!("this is an vector:{v:?}, length: {}", v.len());

    println!("the position 2 from V vector is: {}", v[2]);

    println!("Pop off: {:?}", v.pop());
    println!("this is an vector after pop: {v:?}, length: {}", v.len());

    index_vec[1] = 5;

    println!("after change: {v:?}");

    println!("{index_vec:?}");
}
