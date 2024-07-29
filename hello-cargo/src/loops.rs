pub fn loops() {
    // El siguiente loop es infinito

    // loop {
    //     println!("we loop forever")
    // }

    fn loops_exercise() {
        loop {
            println!("we loop forever");
            break;
        }

        let mut counter = 1;

        let counter_loop = loop {
            counter *= 2;
            if counter > 100 {
                break counter;
            }
        };

        println!("break the loop at counter = {counter_loop:#?}");
    }

    // while

    fn while_exercise() {
        let mut counter = 1;

        while counter < 5 {
            println!("we loop a while...");
            counter += 1;
        }
    }

    fn for_exercise() {
        let big_bids = ["ostrich", "peacock", "stork"];
        for bird in big_bids.iter() {
            println!("the {bird} is a big bird");
        }
    }

    fn iter() {
        for number in 0..10 {
            println!("{}", number * 2);
        }
    }

    loops_exercise();
    while_exercise();
    for_exercise();
    iter()
}
