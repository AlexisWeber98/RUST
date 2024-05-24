pub fn exercise() {
    struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
    }

    #[derive(PartialEq, Debug)]
    //declare enum for car transmission type
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    fn car_factory(
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
    ) -> Car {
        Car {
            color,
            transmission,
            convertible,
            mileage,
        }
    }

    let mut car = car_factory(String::from("Red"), Transmission::Manual, false, 25);

    println!("Created first car: color {:?}, with transmission {:?}, convertible?: {} and with mileage of {:?} km", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Blue"), Transmission::Automatic, false, 20);
    println!("Created second car: color {:?}, with transmission: {:?}, convertible: {:?} and with mileage of: {:?} km", car.color, car.transmission , car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, true, 15);
    println!("Created second car: color {:?}, with transmission: {:?}, convertible: {:?} and with mileage of: {:?} km", car.color, car.transmission , car.convertible, car.mileage);
}
