#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate core;

extern "C" {
    pub fn emscripten_GetProcAddress(
        name: *const std::os::raw::c_char,
    ) -> *const std::os::raw::c_void;
}

include!(concat!(env!("OUT_DIR"), "/emscripten.rs"));
