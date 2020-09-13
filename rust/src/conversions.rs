use std::ffi::{ CStr, CString };

pub unsafe fn char_ptr_to_string(s: *const libc::c_char) -> String {
    return String::from(CStr::from_ptr(s).to_str().unwrap());
}

pub fn str_to_c_char_ptr(s: &str) -> *mut libc::c_char {
    let c_str = CString::new(s.as_bytes()).unwrap_or_default();
    return c_str.into_raw() as *mut libc::c_char;
}
