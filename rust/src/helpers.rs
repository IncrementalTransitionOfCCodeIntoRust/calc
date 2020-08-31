extern crate libc;

use std::ffi::{ CStr, CString };
use rand::Rng;

pub const G: libc::c_double = 1.6180339887498948482;
pub const E: libc::c_double = 2.7182818284590452354;
pub const PI: libc::c_double = 3.14159265358979323846;

const OP_COUNT: usize = 10;
const FUNC_COUNT: usize = 21;

const operators: [char; OP_COUNT] =
    ['+', '-', '*', '/', '%', '^', '$', '~', '_', '!'];

const functions: [&str; FUNC_COUNT] =
    ["sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh",
     "acosh", "atanh", "exp", "floor", "ceil", "round", "log", "ln", "sqrt", "abs",
     "sgn"];


pub fn str_to_c_char_ptr(s: &str) -> *mut libc::c_char {
    let c_str = CString::new(s.as_bytes()).unwrap_or_default();
    return c_str.into_raw() as *mut libc::c_char;
}

pub unsafe fn char_ptr_to_string(s: *const libc::c_char) -> String {
    return String::from(CStr::from_ptr(s).to_str().unwrap());
}

#[no_mangle]
pub extern "C" fn isFunction(s: *mut libc::c_char) -> libc::c_int {
    let mut i = FUNC_COUNT - 1;
    let s_str = unsafe { char_ptr_to_string(s) };

    while functions[i].ne(s_str.as_str()) {
        i -= 1;
    }

    i as libc::c_int
}

#[no_mangle]
pub extern "C" fn isOperator(c: libc::c_uchar) -> libc::c_int {
    let mut i = OP_COUNT - 1;

    while operators[i] != c as char {
        i -= 1;
    }

    i as libc::c_int
}

#[no_mangle]
pub extern "C" fn isSymbol(s: *mut libc::c_char) -> libc::c_double {
    const e: *mut libc::c_char = "e".as_ptr() as *mut libc::c_char;
    const pi: *mut libc::c_char = "pi".as_ptr() as *mut libc::c_char;
    const inf: *const i8 = "inf".as_ptr() as *const i8;
    const rand: *const i8 = "rand".as_ptr() as *const i8;
    const g: *const i8 = "g".as_ptr() as *const i8;

    let s_str = unsafe { char_ptr_to_string(s) };
    match s_str.as_str() {
        "e" => E,
        "pi" => PI,
        "rand" => rand::thread_rng().gen::<libc::c_double>(),
        "inf" => f64::INFINITY as libc::c_double,
        "g" => return G,
        _ => 0.0
    }
}

#[no_mangle]
pub extern "C" fn factorial(mut d: libc::c_double) -> libc::c_double {
    let mut result = 1.0;

    while d > 1.0 {
        result *= d;
        d -= 1.0;
    }

    result
}

#[no_mangle]
pub extern "C" fn toRadians(mut d: libc::c_double) -> libc::c_double {
    d * (PI / 180.0)
}

#[no_mangle]
pub extern "C" fn toDegrees(mut d: libc::c_double) -> libc::c_double {
    d * (180.0 / PI)
}
