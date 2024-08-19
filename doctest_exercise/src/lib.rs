/// Esta función divide dos números.
///  #Ejemplo #1: 10/2 == 5,
/// 
/// ``` 
/// let result = doctest_exercise::div(10,5); 
/// assert_eq!(result, 2);
/// ```
/// 
/// #Ejemplo #2 6/3 =2
/// 
/// ```
/// let result = doctest_exercise::div(6,3);
/// assert_eq!(result,2);
/// ```
/// 
/// #Panics
/// 
/// La función paniquea si el segundo argumento es 0.
/// 
/// ```rust,should_panic
/// //panicos en division por 0
///  doctest_exercise::div(5,0);
/// ```

pub fn div(a:i32,b:i32) -> i32 {
    if b==0 {
        panic!("Dividir po 0 es un error");
    }
    a/b
}

/// Esta función sustrae dos números.
/// 
/// # Ejemplo #1: 9 - 2 = 7
/// 
/// ```
/// let result = doctest_exercise::sub(9,2);
/// assert_eq!(result, 7)
/// ```
/// 
/// # Ejemplo #2 : 6-9 == -3
/// 
/// ```
/// let result = doctest_exercise::sub(6,9);
/// assert_eq!(result,-3);
/// ```

pub fn sub (a:i32, b:i32) -> i32 {
    a - b
}