use rand::Rng;
use factorial::Factorial;
use crate::conversions::char_ptr_to_string;
use crate::definitions::{ G, E, PI, INF, functions, operators };

#[no_mangle]
pub extern "C" fn isFunction(s: *mut libc::c_char) -> libc::c_int {
    let s_str = unsafe { char_ptr_to_string(s) };
    match functions.get(&s_str) {
        Some(position) => *position,
        None => -1
    }
}

#[no_mangle]
pub extern "C" fn isOperator(c: libc::c_uchar) -> libc::c_int {
    match operators.get(&(c as char)) {
        Some(position) => *position,
        None => -1
    }
}

#[no_mangle]
pub extern "C" fn isSymbol(s: *mut libc::c_char) -> libc::c_double {
    let s_str = unsafe { char_ptr_to_string(s) };
    match s_str.as_str() {
        "e" => E,
        "pi" => PI,
        "rand" => rand::thread_rng().gen::<libc::c_double>(),
        "inf" => INF,
        "g" => G,
        _ => 0.0
    }
}

#[no_mangle]
pub extern "C" fn factorial(mut d: libc::c_double) -> libc::c_double {
    if (d > 2.0) { d * factorial(d - 1.0) }
    else { d }
}

#[no_mangle]
pub extern "C" fn toRadians(mut d: libc::c_double) -> libc::c_double {
    d * (PI / 180.0)
}

#[no_mangle]
pub extern "C" fn toDegrees(mut d: libc::c_double) -> libc::c_double {
    d * (180.0 / PI)
}
