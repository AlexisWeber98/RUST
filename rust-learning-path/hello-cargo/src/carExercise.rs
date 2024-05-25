pub fn car_exercise() {
    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Auto,
    }

    #[derive(PartialEq, Debug)]

    enum Age {
        New,
        Used,
    }

    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    fn car_quality(miles: u32) -> (Age, u32) {
        let quality: (Age, u32) = (Age::New, 0);

        return quality;
    }

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        Car {
            color: color,
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    enum Color {
        Blue,
        Green,
        Red,
        Silver,
    }

    let colors = ["Blue", "Green", "Red", "Color", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {} , {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::Auto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Convertible = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {} , {} miles",
        car.age.0, car.roof, car.color, car.color, car.age.1
    );
}
