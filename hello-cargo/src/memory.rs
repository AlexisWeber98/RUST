pub fn memory_lesson() {
    // mascot no es válida en este ambito (contexto, scope)

    fn example() {
        let mascot = String::from("Ferris"); // 'mascot' es válido desde este momento
        println!("mascot: {mascot:?}");

        // Se puede operar con mascot
        let _ferris = mascot;

        // se transfiere la propiedad de mascot al enlace (variable) 'ferris'
        // 'mascot'ya no es válida, ya no se puede operar con 'mascot', porque su valor se movió a 'ferris

        println!("{}", mascot); // el compilador falla porque 'mascot' ya no es válida, no posee valor alguno
    }

    //este ámbito (scope) ya terminó, por lo que mascot ya no es operable, mascot no existe fuera de los '{}'
    // y la informacion de memoria de datos para 'mascot'se libera, mismo así con 'ferris'

    // println!("{mascot:?}");  esto no se puede ya que println!() no conoce 'mascot'

    example()
}
