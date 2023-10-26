use std::ffi::{c_char, CStr, CString};

static mut RESULT: &mut [u8] = &mut [0; 5000];

use uroborosql_fmt::{config::Config, format_sql_with_config};

#[no_mangle]
pub unsafe extern "C" fn getResultAddress() -> *const u8 {
    &RESULT[0]
}

/// Formats SQL code given as char pointer `src` by WASM (JavaScript).
///
/// # Safety
///
/// This is unsafe because it uses unsafe function
/// [`CStr::from_ptr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html#method.from_ptr).
#[export_name = "format_sql"]
#[no_mangle]
pub unsafe extern "C" fn format_sql_for_wasm(src: *mut c_char, config_json_str: *mut c_char) {
    let src = CStr::from_ptr(src).to_str().unwrap().to_owned();

    let config_json_str = CStr::from_ptr(config_json_str).to_str().unwrap();
    let config = Config::from_json_str(config_json_str).unwrap();

    // TODO: error handling
    let result = format_sql_with_config(&src, config).unwrap();

    CString::new(result)
        .unwrap()
        .as_bytes_with_nul()
        .iter()
        .enumerate()
        .for_each(|(i, x)| unsafe {
            RESULT[i] = *x;
        });

    // let format_result = CString::new(result).unwrap().into_raw();
    // let error_msg = CString::new("error_msg").unwrap().into_raw();

    // [format_result, error_msg].as_mut_ptr()
}

/// Free the string `s` allocated by Rust.
///
/// # Safety
///
/// This is unsafe because it uses the unsafe function
/// [`CString::from_war()`](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_raw).
#[no_mangle]
pub unsafe extern "C" fn free_format_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}
