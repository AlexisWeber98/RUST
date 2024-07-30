pub fn errs() {
    // Panic se utiliza para indicar de un punto donde no se puede rescatar el error y el código se termina (se ejecuta al final para que no detenga la ejecución de el resto del código)

    pub fn panic() {
        panic!("Farewell!")
    }

    //Option se utiliza para tratar una ausencia, cuando un dato puede o no estar presente, una forma de remplazar el undefined o el null

    // dentro de los " < > " va el tipo de dato, el tipo <T> es el más genérico

    enum _Option<T> {
        None,    // valor no existe
        Some(T), // valor existe
    }

    //  None y Some no son tipos sino variantes del tipo Option<T>, lo que significa, entre otras cosas, que las funciones no pueden tomar Some o None como argumentos, sino solo Option<T>.

    //Para evitar que Rust "Panicked" con cosas que no existen en los vectores (índices falopa como 99 en un arreglo de 3 posiciones) usamos get()
    //get() devuelve un Option<T> que puede ser None o Some(T)

    let fruits = vec!["banana", "manzana", "naranja", "frutilla"];

    let first = fruits.get(0);
    println!("{first:?}");

    let third = fruits.get(2);
    println!("{third:?}");

    let non_exist = fruits.get(99);
    println!("{non_exist:?}");

    // se puede usar MATCH para controlar el flujo del programa mediante el aprovisionamiento de patrones. Cuando match encuentra un patrón de coincidencia, ejecuta el código que se proporciona con ese patrón.

    for &index in [0, 2, 3, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("La {fruit_name:?} Es riquísima"),
            None => println!("No se de que me hablas rey :("),
        }
    }

    //En el código anterior, se recorren en iteración los mismos índices del ejemplo previo (0, 2 y 99) y, luego, se usa cada uno de ellos para recuperar un valor del vector fruits mediante la expresión fruits.get(index).

    for &index in [0, 2, 3, 99].iter() {
        match fruits.get(index) {
            Some(&"frutilla") => println!("Aguante la frutilla"),
            Some(fruit_name) => println!("La {fruit_name:?} Es riquísima"),
            None => println!("No hay pa"),
        }
    }

    // Rust ofrece una manera cómoda de probar si un valor se ajusta a un solo patrón.

    let a_number: Option<u8> = Some(7);

    match a_number {
        Some(7) => println!("the number is 7"),
        _ => {}
    }

    // con la notación " _ " ignoramos el None y solo nos quedamos con el valor que nos interesa, pero podemos hacerlo mejor...

    let a_number: Option<u8> = Some(10);

    if let Some(10) = a_number {
        println!("the number is 10");
    }

    //Puede intentar acceder al valor interno de un tipo Option directamente mediante el método unwrap. Sin embargo, tenga cuidado, ya que este método emitirá una alerta de pánico si la variante es None.

    //Por ejemplo:

    let gift = Some("Candy");
    assert_eq!(gift.unwrap(), "Candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "Candy"); // esto arroja Panic!

    // El método expect hace lo mismo que unwrap, pero emite un mensaje de pánico personalizado que su segundo argumento proporciona:

    let a = Some("Value");
    let exist = assert_eq!(a.expect("fruits are healthy"), "Value");
    println!("hello: {exist:?}");

    // let b: Option<&str> = None;
    // let non_exist = b.expect("fruits are healthy"); //arroja "fruits are healthy" y detiene la ejecución
    // println!("bye: {non_exist:?}");

    //Como estas funciones pueden emitir alertas de pánico, no se recomienda usarlas. Considere mejor uno de los siguientes enfoques:

    //Use la coincidencia de patrones y administre el caso None explícitamente.
    //Llame a métodos similares que no emiten alertas de pánico, como unwrap_or, que devuelve un valor predeterminado si la variante es None o el valor interno si la variante es Some(value).

    let some = assert_eq!(Some("dog").unwrap_or("cat"), "dog");

    println!("some: {some:?}");

    let none = assert_eq!(None.unwrap_or("cat"), "cat");
    println!("none: {none:?}");

    // todo es válido

    panic()
}
