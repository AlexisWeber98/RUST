

// ------------------------------------- Testing --------------------------------------- //
mod doc_test;

fn add (a:i32, b:i32) -> i32 {
    a + b
}

#[test]
fn add_works(){
    assert_eq!(add (1,2), 3);
    assert_eq!(add(3,1), 4);
    assert_eq!(add(2, 5), 7);
}


// error
/*
#[test]
fn add_fails() {
    assert_eq!(add(5,5), 8)
}

*/ 


//  errores esperados


#[test]
#[should_panic]
fn add_fails(){
    assert_eq!(add(10,22),12);
}


// ignore

// El atributo [ignore] se puede escribir opcionalmente junto con un motivo para la omisión de la prueba.

#[test]
#[ignore = "Aún no revisado por el equipo de Q.A"]
fn add_negatives(){
    assert_eq!(add(-22, -22), 44);
}

// ---------------------------- MÓDULO DE TESTING -------------------------------// 

// La mayoría de las pruebas unitarias se incluyen en un submódulo con el atributo #[cfg(test)].


fn add2(a: i32, b: i32) -> i32 {
    a +b 
}

#[cfg(test)]
mod add_function_test {
    use super::*;
    
//  La declaración use super::*; es necesaria para que el código del módulo add_function_tests pueda acceder a la función add en el módulo externo.
    
    #[test]
    fn add_works(){
        assert_eq!(add2( 1, 2 ), 3 );
        assert_eq!(add2( 10, 12 ), 22 );
        assert_eq!(add2( 5, -2 ), 3 );
    }
    
    #[test]
    #[should_panic]
    fn add_fails2(){
        assert_eq!(add2( 2, 3 ), 7 );
    } 
    
    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add( -2, -2 ), 4)
    }
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn is_true_when_even() {
        assert_eq!(is_even(2), true)
    }
    
    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(3), false)
    }
}

// lib.rs = pruebas documentales

fn main() {}