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
}
