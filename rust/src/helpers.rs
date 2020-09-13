extern crate libc;

use std::ffi::CStr;
use rand::Rng;
use factorial::Factorial;

pub const G:  libc::c_double = 1.6180339887498948482;
pub const E:  libc::c_double = 2.7182818284590452354;
pub const PI: libc::c_double = 3.14159265358979323846;

const OP_COUNT:   usize = 10;
const FUNC_COUNT: usize = 21;

const operators: [char; OP_COUNT] =
    ['+', '-', '*', '/', '%', '^', '$', '~', '_', '!'];

const functions: [&str; FUNC_COUNT] =
    ["sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh",
     "acosh", "atanh", "exp", "floor", "ceil", "round", "log", "ln", "sqrt", "abs",
     "sgn"];

unsafe fn char_ptr_to_string(s: *const libc::c_char) -> String {
    return String::from(CStr::from_ptr(s).to_str().unwrap());
}

#[no_mangle]
pub extern "C" fn isFunction(s: *mut libc::c_char) -> libc::c_int {
    let s_str = unsafe { char_ptr_to_string(s) };
    return functions.iter().position(|&st| st == s_str).unwrap() as libc::c_int
}

#[no_mangle]
pub extern "C" fn isOperator(c: libc::c_uchar) -> libc::c_int {
    return operators.iter().position(|&ch| ch == c as char).unwrap() as libc::c_int
}

#[no_mangle]
pub extern "C" fn isSymbol(s: *mut libc::c_char) -> libc::c_double {
    let s_str = unsafe { char_ptr_to_string(s) };
    match s_str.as_str() {
        "e" => E,
        "pi" => PI,
        "rand" => rand::thread_rng().gen::<libc::c_double>(),
        "inf" => f64::INFINITY as libc::c_double,
        "g" => G,
        _ => 0.0
    }
}

#[no_mangle]
pub extern "C" fn factorial(mut d: libc::c_double) -> libc::c_double {
    return (d as u32).factorial() as libc::c_double
}

#[no_mangle]
pub extern "C" fn toRadians(mut d: libc::c_double) -> libc::c_double {
    d * (PI / 180.0)
}

#[no_mangle]
pub extern "C" fn toDegrees(mut d: libc::c_double) -> libc::c_double {
    d * (180.0 / PI)
}
