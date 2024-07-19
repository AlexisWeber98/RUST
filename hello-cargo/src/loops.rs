pub fn loops() {
    // El siguiente loop es infinito

    // loop {
    //     println!("we loop forever")
    // }

    loop {
        println!("we loop forever");
        break;
    }

    let mut counter = 1;

    let counter_loop = loop {
        counter *= 2;
        if counter > 100 {
            break;
        }
    };
}
