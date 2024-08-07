use std::fmt;
use std::iter::Iterator;

pub fn main_rasgos() {
    pub fn main_rasgos1() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let boolean = Point { x: true, y: false };
        println!(
            "Rasgo Struct boolean is:  x= {:?}, y = {:?}",
            boolean.x, boolean.y
        );

        let integer = Point { x: 1, y: 7 };
        println!(
            "Rasgo Struct Integer is:  x= {:?}, y = {:?}",
            integer.x, integer.y
        );

        let float = Point { x: 1.2, y: 4.5 };
        println!(
            "Rasgo Struct Float is:  x= {:?}, y = {:?}",
            float.x, float.y
        );

        let string_slice = Point {
            x: "high",
            y: "slow",
        };

        println!("Rasgo Struct string is: {string_slice:?}");
    }

    main_rasgos1();

    //El código anterior define una estructura Point<T>. Esta estructura contiene cualquier tipo ( T) de valores x y y.

    //let wont_work = Pint{x:25, y = true }; si queremos que Point<T> tenga tipos diferentes no compila

    // si queremos que x e y puedan tener tipos diferentes debemos definirlo, por ejemplo Point<T,U>

    fn main_points() {
        #[derive(Debug)]
        struct Points<T, U> {
            x: T,
            y: U,
        }
        let inter_and_boolean = Points { x: 5, y: false };
        println!(
            "Rasgos con diferente tipado integer y boolean: x= {:?}, y = {:?}",
            inter_and_boolean.x, inter_and_boolean.y
        );

        let string_and_float = Points { x: "hello", y: 5.5 };
        println!(
            "Rasgos con diferente tipado 2 (string y float): x= {:?}, y = {:?}",
            string_and_float.x, string_and_float.y
        );

        let integer_and_float = Points { x: 10, y: 4.4 };
        println!(
            "Rasgos con diferente tipado 3 (integer y float): x= {:?}, y = {:?}",
            integer_and_float.x, integer_and_float.y
        );

        let both_integer = Points { x: 11, y: 12 };
        println!(
            "Rasgos con diferetnte tipado 4 (ambos integer):x= {:?}, y = {:?}",
            both_integer.x, both_integer.y
        );

        let both_boolean = Points { x: true, y: false };
        println!(
            "Rasgos con diferente tipado 5 (ambos boolenas): x= {:?}, y = {:?}",
            both_boolean.x, both_boolean.y
        );
    }

    main_points();

    /*
     Todos los tipos Points anteriores tienen distintos tipos concretos. Por orden:

    * Point<integer, bool>
    * Point<f64, &'static str>
    * Point<integer, f64>
    * Point<integer, integer>
    * Point<bool, bool>

     */

    trait Area {
        fn area(&self) -> f64;
    }

    // Aquí, se declara un rasgo mediante la palabra clave trait y luego el nombre del rasgo, que en este caso es Area.

    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        heith: f64,
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            use std::f64::consts::PI;
            PI * self.radius.powf(2.0)
        }
    }

    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.heith
        }
    }

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        heith: 20.5,
    };

    println!(
        "Circle: {}, Rectangtle: {}",
        circle.area(),
        rectangle.area()
    );

    // esto dara un error //

    /*
    struct Point1 {
        x: i32,
        y: i32,
    }

    fn main_point1() {
        let p1 = Point1 { x: 1, y: 2 };
        let p2 = Point1 { x: 4, y: -3 };

        if p1 == p2 {             // No se puede comparar los dos valores Point1
            println!("equal");
        } else {
            println!("not equal")
        }

        println!("{}", p1);   //  ¡no puedo imprimir usando el especificador de formato '{}'!
        println!(":?", p1);    // no se puede imprimir usando '{:?}' ¡especificador de formato!
    }

    Este código no se compila porque el tipo Point no implementa los rasgos siguientes:

    * El rasgo Debug, que permite dar formato a un tipo mediante el especificador de formato {:?}, se usa en un contexto de depuración orientado al programador.

    * El rasgo Display, que permite dar formato a un tipo mediante el especificador de formato {}, es similar a Debug. Pero Display es más adecuado para la salida orientada al usuario.

    * El rasgo PartialEq, que permite comparar la igualdad de los implementadores.

    */

    // USO DE DERIVACIÓN

    // el compilador de Rust puede implementar automáticamente los rasgos Debug y PartialEq mediante el atributo #[derive(Trait)] si cada uno de sus campos implementa el rasgo:

    #[derive(Debug, PartialEq)]
    struct Point2 {
        x: i32,
        y: i32,
    }

    /*
       Nuestro código seguirá sin compilarse porque la biblioteca estándar de Rust no ofrece la implementación automática del rasgo Display, porque está pensado para los usuarios finales. Pero si esa línea se convierte en comentario, el código genera esta salida:

       not equal!
       Point { x: 1, y: 2 }
    */

    //  No obstante, podemos implementar el rasgo Display para nuestro tipo por nosotros mismos:

    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    fn main_point2() {
        let p1 = Point2 { x: 1, y: 2 };
        let p2 = Point2 { x: 4, y: -3 };

        if p1 == p2 {
            // No se puede comparar los dos valores Point1
            println!("equal");
        } else {
            println!("not equal")
        }

        println!("{}", p1); //  ¡no puedo imprimir usando el especificador de formato '{}'!
        println!("{:?}", p2); // no se puede imprimir usando '{:?}' ¡especificador de formato!
    }

    main_point2();

    // Supongamos que estamos escribiendo una aplicación web y queremos tener una interfaz para serializar valores en el formato JSON. Podríamos escribir un rasgo similar al siguiente

    trait AsJson {
        fn as_json(&self) -> String;
    }

    //Después, podríamos escribir una función que acepte cualquier tipo que implemente el rasgo AsJson. Se escriben como impl seguido de un conjunto de límites de rasgo.

    fn send_data_as_json(value: &impl AsJson) {
        println!("Sending JSON to server...");
        println!("-> {}", value.as_json());
        println!("Done!\n");
    }

    // otra forma de escribir la misma funcion pero con distinta sintaxis es indicar explicitamente que T  es un tipo generico que debe implementar el rasgo AsJson:

    //  fn send_data_as_json2<T: AsJson>(value: &T) {...}

    struct Person {
        name: String,
        last_name: String,
        age: u8,
    }

    struct Dog {
        name: String,
        color: String,
        likes_petting: bool,
    }

    struct Cat {
        name: String,
        sharp_claws: bool,
    }

    impl AsJson for Person {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "person", "name": "{}", "lastName": "{}, "age": "{}" }}"#,
                self.name, self.last_name, self.age
            )
        }
    }

    impl AsJson for Dog {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "dog", "name": "{}", "color": "{}", "likes_petting": "{}" }}"#,
                self.name, self.color, self.likes_petting
            )
        }
    }

    impl AsJson for Cat {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type" : "cat", "name": "{}", sharp_claws": "{}" }}"#,
                self.name, self.sharp_claws
            )
        }
    }

    // Ahora que tanto Person como Dog implementan el rasgo AsJson, se pueden usar como parámetros de entrada para la función send_data_as_json.

    fn main_as_json() {
        let laura = Person {
            name: String::from("Laura"),
            last_name: String::from("Vallejos"),
            age: 22,
        };

        let fido = Dog {
            name: String::from("Fido"),
            color: String::from("Black"),
            likes_petting: true,
        };

        let kitty = Cat {
            name: String::from("Kitty"),
            sharp_claws: false,
        };

        send_data_as_json(&laura);
        send_data_as_json(&fido);
        send_data_as_json(&kitty);
    }

    main_as_json();

    // ITERADORES

    // El núcleo de este rasgo (Iterator) tiene el siguiente aspecto:

    // Definir la estructura Counter
    #[derive(Debug)]
    struct Counter {
        length: usize,
        count: usize,
    }

    impl Counter {
        fn new(length: usize) -> Counter {
            Counter { count: 0, length }
        }
    }

    // Implementar el `trait` Iterator de la biblioteca estándar para Counter
    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count <= self.length {
                Some(self.count)
            } else {
                None
            }
        }
    }

    fn iterator() {
        let mut counter = Counter::new(6);
        println!("\nCounter just created: {:#?}", counter);

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), Some(6));
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None); // further calls to `next` will return `None`
        assert_eq!(counter.next(), None);

        println!("Counter exhausted: {:#?}", counter);
    }

    iterator();

    fn main_iterator() {
        // Usar el iterador en un bucle `for`
        for number in Counter::new(10) {
            println!("{}", number);
        }

        // Usar el método `sum` del iterador
        let sum_until_10: usize = Counter::new(10).sum();
        assert_eq!(sum_until_10, 55);

        // Usar el método `map` del iterador
        let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
        assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
    }

    main_iterator();

    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        pub fn new(value: T) -> Self {
            Container { value }
        }
    }

    fn generic_exercise() {
        assert_eq!(Container::new(42).value, 42);
        assert_eq!(Container::new(3.14).value, 3.14);
        assert_eq!(Container::new("Foo").value, "Foo");
        assert_eq!(
            Container::new(String::from("Bar")).value,
            String::from("Bar")
        );
        assert_eq!(Container::new(true).value, true);
        assert_eq!(Container::new(-12).value, -12);
        assert_eq!(Container::new(Some("text")).value, Some("text"));
    }

    generic_exercise();

    struct Groups<T> {
        inner: Vec<T>,
    }

    impl<T> Groups<T> {
        fn new(inner: Vec<T>) -> Self {
            Groups { inner }
        }
    }

    impl<T: PartialEq> Iterator for Groups<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            // if the inner vector is empty, we are done
            if self.inner.is_empty() {
                return None;
            }

            // lets check the span of equal items
            let mut cursor = 1;
            let first = &self.inner[0];
            for element in &self.inner[1..] {
                if element == first {
                    cursor += 1;
                } else {
                    break;
                }
            }

            // we use the `Vec::drain` to extract items up until the cursor
            let items = self.inner.drain(0..cursor).collect();

            // return the extracted items
            Some(items)
        }
    }

    fn iterator_exercise() {
        let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
        // groups:     |->|---->|->|->|--->|----------->|--->|

        assert_eq!(
            Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
            vec![
                vec![4],
                vec![1, 1],
                vec![2],
                vec![1],
                vec![3, 3],
                vec![-2, -2, -2],
                vec![5, 5],
            ]
        );

        let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
        // groups:      |->|---->|---->|----|->|----->|->|

        assert_eq!(
            Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
            vec![
                vec![1],
                vec![2, 2],
                vec![1, 1],
                vec![2, 2],
                vec![3],
                vec![4, 4],
                vec![3],
            ]
        )
    }

    iterator_exercise()
}
