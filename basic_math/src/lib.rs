/// Generalmente, la primera línea es un breve resumen que describe la función.
///
/// Las siguientes líneas presentan documentación detallada.
/// Los bloques de código comienzan con ticks triples. El código tiene un implícito `fn main()` inside and `extern crate <cratename>`,
/// lo que significa que puedes empezar a escribir código.
///
/// ```
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}



