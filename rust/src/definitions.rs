use lazy_static::lazy_static;
use std::collections::HashMap;

pub const G:    libc::c_double = 1.6180339887498948482;
pub const E:    libc::c_double = 2.7182818284590452354;
pub const PI:   libc::c_double = 3.14159265358979323846;
pub const INF:  libc::c_double = f64::INFINITY;

lazy_static! {
    pub static ref operators: HashMap<char, libc::c_int> = {
        let mut map = HashMap::new();
        map.insert('+', 0);
        map.insert('-', 1);
        map.insert('*', 2);
        map.insert('/', 3);
        map.insert('%', 4);
        map.insert('^', 5);
        map.insert('$', 6);
        map.insert('~', 7);
        map.insert('_', 8);
        map.insert('!', 9);
        map
    };
}

lazy_static! {
    pub static ref functions: HashMap<String, libc::c_int> = {
        let mut map = HashMap::new();
        map.insert(String::from("sin"),   0);
        map.insert(String::from("cos"),   1);
        map.insert(String::from("tan"),   2);
        map.insert(String::from("asin"),  3);
        map.insert(String::from("acos"),  4);
        map.insert(String::from("atan"),  5);
        map.insert(String::from("sinh"),  6);
        map.insert(String::from("cosh"),  7);
        map.insert(String::from("tanh"),  8);
        map.insert(String::from("asinh"), 9);
        map.insert(String::from("acosh"), 10);
        map.insert(String::from("atanh"), 11);
        map.insert(String::from("exp"),   12);
        map.insert(String::from("floor"), 13);
        map.insert(String::from("ceil"),  14);
        map.insert(String::from("round"), 15);
        map.insert(String::from("log"),   16);
        map.insert(String::from("ln"),    17);
        map.insert(String::from("sqrt"),  18);
        map.insert(String::from("abs"),   19);
        map.insert(String::from("sgn"),   20);
        map
    };
}
