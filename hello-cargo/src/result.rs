pub fn result() {
    //     fn err_enum() {

    //     enum Result<T, E> {
    //         Ok(T),  // un valor T
    //         Err(E), // un error tipo E
    //     }
    // }

    #[derive(Debug)]
    struct DivisionByZeroError;

    fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 {
            Err(DivisionByZeroError)
        } else {
            Ok(dividend / divisor)
        }
    }

    fn divide() {
        println!("primera division: {:?}", safe_division(9.0, 3.0));
        println!("segunda division: {:?}", safe_division(4.0, 0.0));
        println!("tercera division: {:?}", safe_division(0.0, 2.0));
    }

    divide();
}
