pub fn references() {
    //Las referencias permiten "tomar prestados" valores sin convertirse en propietario de ellos.

    let greeting = String::from("Hello");
    let _greeting_reference = &greeting; // con la notación '&' como prefijo de la asignacion tomamos prestado `greeting` pero los datos  de la cadena todavía son propiedad de `greeting

    println!("greeting : {greeting:?}"); //todavía podemos usar 'greeting' :)

    // greeting se ha tomado prestado con el símbolo de referencia (&). La variable greeting_reference era de tipo de referencia de cadena (&String). Dado que solo se ha tomado prestado greeting y no se ha movido la propiedad, greeting se podría seguir usando después de crear greeting_reference.

    fn print_greeting(message: &String) {
        println!("Greeting Message: {message:?}");
    }

    fn caller() {
        let greeting = String::from("Hello");
        print_greeting(&greeting); // `print_greeting` toma un `&String` que no es un `String` de propiedad, por lo que tomamos prestad`greeting` con `&`
        print_greeting(&greeting); // dado que greting no fue movido a print_greeting podemos usarla nuevamente

        // El préstamo permite usar un valor sin aceptar su propiedad completa. Pero, como vamos a ver, el préstamo de un valor significa que no se puede hacer todo lo que se podría hacer con un valor cuya propiedad fuera totalmente nuestra.
    }

    caller();

    //¿Qué sucede si se intenta mutar un valor que se ha tomado prestado?

    // fn change(message:&String) {
    //     message.push_str("!"); // intentamos añadirle "!" al final del mensaje
    // }

    // fn call_change(){
    //     let greeting = String::from("Hola Cambio");
    //     change(&greeting);
    // }

    // esto dara un error de compilacion

    fn change(message: &mut String) {
        message.push_str("!"); // intentamos añadirle "!" al final del mensaje
        println!("change message: {message:?}");
    }

    fn call_change() {
        let mut greeting = String::from("Hola Cambio");
        change(&mut greeting);
    }

    call_change();

    // Las referencias inmutables y mutables difieren de una manera adicional que tiene efectos radicales sobre cómo se compilan los programas de Rust

    // el código debe implementar CUALQUIERA de de las definiciones siguientes, pero NO las dos al mismo tiempo:

    /*  Una o más referncias inmutables ( &T) */
    /* Exactamnete UNA referenciua mutable ( &mut /) */

    fn ref_without_defs_avalaibles() {
        let mut value = String::from("Hello");

        let ref1 = &mut value;
        // let ref2 = &mut value; // da error porque se usando una referencia mutable mas de una vez

        println!("{ref1:?}");
        // println!("{ref2:?}"); // da error porque se esta usando una referncia mutable más de una vez

        //Incluso podemos intentar mezclar referencias inmutables con referencias mutables, pero el compilador seguirá dando error:

        let mut value2 = String::from("hello cargo");

        //let ref1_value2 = &value2;                  Se comenta para que no rompa el compilador
        let ref2_value2 = &mut value2;

        // println!("Reference 1: {ref1_value2:?}");  Se comenta para que no rompa el compilador
        println!("Reference 2: {ref2_value2:?}")
    }

    ref_without_defs_avalaibles();

    // El uso de referncias plate el siguiente problema:
    // el elemento referenciado no hace seguimiento de todas las referncias, entonces cando se anuola este referenciado
    // ¿cómo se puede tener la seguridad de que no haya referencias que apunten a la memoria ya liberada y, por tanto, no válida?

    // a esto se le conoce como "puntero colgante", en Rust esto no existe

    //La respuesta de Rust a esta pregunta son las duraciones. Permiten que Rust garantice la seguridad de la memoria sin los costos de rendimiento de la recolección de elementos no utilizados.

    //Considere el siguiente fragmento de código, que intenta utilizar una referencia cuyo valor ha salido del ámbito:

    /*  fn suspend_pointer() {
        let x;

        {
            let y = 42;
            x = &y; // Almacenamos una referencia a `y` en `x` pero `y` está a punto de ser eliminado
        }
        println!("x: {x:?}") // X hace referncia a Y, pero Y fue eliminado
    }
    */

    // este error pasa porque se colocó un valor mientras estaba prestado, "y" se colocaal final del ámbito interno, pero "x"lo toma prestado hasta la llama de prinln!()
    // "x" sigue siendo válido para el ámbito externo, porque su ámbito es mayor, es decir que "vive más tiempo"

    //Este es el mismo fragmento de código con dibujos en torno a cada duración variable. Hemos asignado un nombre a cada duración:

    // 'a es la anotación de duración de nuestro valor x.
    // 'b es la anotación de duración de nuestro valor y.

    /*

    fn main() {
        let x;                // ---------+-- 'a
        {                     //          |
            let y = 42;       // -+-- 'b  |
            x = &y;           //  |       |
        }                     // -+       |
        println!("x: {}", x); //          |
    }

    */

    // Aquí podemos ver que el bloque de duración 'b interno es más corto que el bloque 'a externo.

    fn durations() {
        let magic1 = String::from("Abracadabra!");
        let magic2 = String::from("Shazam!");

        let result = longuest_word(&magic1, &magic2);
        println!("The longest magic word is: {result:?}");
    }

    /*
       fn lonuest_word (x: &String, y :&String) -> &String {
           if x.len() > y.len() {
               x
           } else {
               y
               }
           }
       }

    Éste código no se puede conpilar, Rust no puede decir si la referencia que se devuelve hace referencia a x o a y. Nosotros tampoco. Por lo tanto, para ayudar a identificar cuál es la referencia, anote el tipo de valor devuelto con un parámetro genérico que represente la duración.

    El comprobador de préstamos tampoco puede determinar si la referencia será una válida. Desconoce cómo la duración de los parámetros de entrada se relaciona con la duración del valor devuelto. Esta ambigüedad es el motivo por el que es necesario anotar las duraciones manualmente.

    Afortunadamente, el compilador nos ha proporcionado una sugerencia sobre cómo corregir este error. Podemos agregar parámetros de duración genéricos a la signatura de función. Estos parámetros definen la relación entre las referencias para que el comprobador de préstamos pueda realizar su análisis:
    */

    fn longuest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    //En la signatura, el valor devuelto y todas las referencias de parámetro deben tener la misma duración. Por lo tanto, use el mismo nombre de duración, por ejemplo 'a. Luego, agregue el nombre a cada referencia de la signatura de función.

    // Asegúrese de declarar parámetros de duración genéricos entre corchetes angulares y agregue la declaración entre la lista de parámetros y el nombre de la función.
    durations();

    // En este caso, no hay nada especial acerca del nombre 'a. Sería igual de correcto usar cualquier otra palabra, como 'response o 'program. Lo importante que debe tenerse en cuenta es que todos los parámetros y el valor devuelto estarán activos al menos mientras esté vigente la duración asociada a cada uno de ellos
    //

    //Cada vez que un struct o una enumeración contienen una referencia en uno de sus campos, debemos anotar esa definición de tipo con la duración de cada referencia que lleve a cabo con ella.

    fn notation_time_types() {
        #[derive(Debug)]
        struct Highlight<'document>(&'document str); /* colocamos el nombre del parámetro de duracion entre "< >" después del nombre de la estructura.
                                                     Esta instancia de Highlight no puede durar más que la referencia de su campo debido a la declaración. */

        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        println!("{:?}", fox);
        println!("{:?}", dog);
    }

    notation_time_types();

    // #[derive(Debug)]
    // struct Highlight<'document>(&'document str);

    // fn erase(_: String) {};

    // fn notation_time_types() {
    //     let text = String::from("The quick brown fox jumps over the lazy dog.");
    //     let fox = Highlight(&text[4..19]);
    //     let dog = Highlight(&text[35..43]);

    //     erase(text);

    //     println!("prueba1: {fox:?}");
    //     println!("prueba2: {dog:?}");
    // }

    // notation_time_types();

    fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
        vector.push(String::from(value));
        vector.get(vector.len() - 1).unwrap()
    }

    pub fn execute() {
        let name1 = "Joe";
        let name2 = "Chirs";
        let name3 = "Anne";

        let mut names = Vec::new();

        assert_eq!("Joe", copy_and_return(&mut names, &name1));
        assert_eq!("Chris", copy_and_return(&mut names, &name2));
        assert_eq!("Anne", copy_and_return(&mut names, &name3));

        assert_eq!(
            names,
            vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
        )
    }

    execute();
}
