use std::ffi::CStr;
use lazy_static::lazy_static;
use libc::{c_char};
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}


#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {}!", name);
}

#[no_mangle]
pub extern "C" fn whisper(message: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    println!("({})", message);
}

#[no_mangle]
pub extern "C" fn run_tokio() {
    Runtime::spawn(&RUNTIME, async move {
        loop {
            println!("Hi from tokio!");
            tokio::time::sleep(core::time::Duration::from_secs(2)).await;
        }
    });
}

#[no_mangle]
pub extern "C" fn test_pointer(ptr: *const c_char) {
    println!("received a nice pointer: {:?}", ptr);
}

// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {

    use std::ffi::CString;
    use super::*;

    // This is meant to do the same stuff as the main function in the .go files
    #[test]
    fn simulated_main_function () {
        hello(CString::new("world").unwrap().into_raw());
        whisper(CString::new("this is code from Rust").unwrap().into_raw());
    }
}
