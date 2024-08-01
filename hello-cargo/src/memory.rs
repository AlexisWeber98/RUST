pub fn memory_lesson() {
    // mascot no es válida en este ambito (contexto, scope)

    fn example() {
        let mascot = String::from("Ferris"); // 'mascot' es válido desde este momento
        println!("mascot: {mascot:?}");

        // Se puede operar con mascot
        let _ferris = mascot;

        // se transfiere la propiedad de mascot al enlace (variable) 'ferris'
        // 'mascot'ya no es válida, ya no se puede operar con 'mascot', porque su valor se movió a 'ferris

        // println!("{}", mascot); el compilador falla porque 'mascot' ya no es válida, no posee valor alguno

        //este ámbito (scope) ya terminó, por lo que mascot ya no es operable, mascot no existe fuera de los '{}'
        // y la informacion de memoria de datos para 'mascot'se libera, mismo así con 'ferris'

        // println!("{mascot:?}");  esto no se puede ya que println!() no conoce 'mascot'

        fn process(input: String) {
            println!("ejercicio de asignacion de propiedad : {input:?}")
        }

        fn caller() {
            let s = String::from("Pepe");
            process(s);
            //process(s); // error: el valor de 's' fue movido a process, s ya no tiene asignaion, no se puede volver operar con 's'
        }

        caller();

        // Los tipos simples como los número son tipos de copia. Implementan el rasgo Copy, lo que significa que se copian en lugar de moverse.
        // La copia de números es muy económica, por lo que tiene sentido que estos valores se copien. La copia de cadenas o vectores, u otros tipos complejos, puede ser costosa, por lo que no implementan el rasgo Copy y, en su lugar, se mueven.

        fn process_number(input: u32) {
            println!("Ejercicio de Copy implícito: {input:?}");
        }

        fn called_number() {
            let n = 1u32;
            process_number(n);
            process_number(n); // no hay error, porque el valor de asugnaciona 'n' no se movio, se copió
        }

        called_number();

        //Una manera de solucionar los errores que se han visto en el ejemplo anterior consiste en copiar explícitamente los tipos antes de moverlos: lo que se conoce como clonación en Rust. Una llamada a .clone duplica la memoria y genera un nuevo valor. El nuevo valor se mueve, lo que significa que todavía se puede usar el valor anterior.

        fn process_copy(s: String) {
            println!("el valor de asignacion copiado es : {s:?}")
        }

        fn called_copy() {
            let s = String::from("Hola Pibe");

            process_copy(s.clone()); // se clona el valor de 's' y esa copia se envía (mueve) a la funcion
            process_copy(s) // se envía a la función moviendo el valor de la asignación original
        }
        called_copy();
        
        // Este enfoque puede resultar útil, aunque puede ralentizar el código, ya que cada llamada a clone realiza una copia completa de los datos. Este método a menudo incluye asignaciones de memoria u otras operaciones costosas. Estos costos se pueden evitar si los valores "se toman prestados" mediante referencias
    }

    example()
}
