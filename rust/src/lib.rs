#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod helpers;

#[cfg(test)]
mod tests {
    use crate::helpers::*;

    #[test]
    fn is_function_works() {
        assert_eq!(isFunction(str_to_c_char_ptr("sin")), 0);
        assert_eq!(isFunction(str_to_c_char_ptr("round")), 15);
        assert_eq!(isFunction(str_to_c_char_ptr("sgn")), 20);
    }

    #[test]
    fn is_operator_works() {
        assert_eq!(isOperator('+' as u8), 0);
        assert_eq!(isOperator('^' as u8), 5);
        assert_eq!(isOperator('!' as u8), 9);
    }

    #[test]
    fn is_symbol_works() {
        assert_eq!(isSymbol(str_to_c_char_ptr("e")), E);
        assert_eq!(isSymbol(str_to_c_char_ptr("pi")), PI);
        assert_eq!(isSymbol(str_to_c_char_ptr("g")), G);
    }

    #[test]
    fn factorial_works() {
        assert_eq!(factorial(4.0), 24.0);
    }

    #[test]
    fn to_radians_works() {
        assert_eq!(toRadians(180.0), PI);
    }

    #[test]
    fn to_degrees_works() {
        assert_eq!(toDegrees(PI), 180.0);
    }
}
