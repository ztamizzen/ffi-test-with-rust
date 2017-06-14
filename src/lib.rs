// From http://jakegoulding.com/rust-ffi-omnibus/objects/
extern crate libc;
extern crate primal;

use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::iter;
use std::slice;
use std::convert::From;
use std::collections::HashMap;

// ============ Integers ============
#[no_mangle]
pub extern fn addition(a: uint32_t, b: uint32_t) -> uint32_t {
    a + b
}

// ============ String arguments ============
#[no_mangle]
pub extern fn chars_counter(s: *const c_char) -> uint32_t {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as uint32_t
}

// ============ Slice return values ============
#[no_mangle]
pub extern fn theme_song_generate(length: uint8_t) -> *mut c_char {
    let mut song = String::from("💣 ");
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn theme_song_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

// ============ Slice arguments ============
#[no_mangle]
pub extern fn sum_of_even(n: *const uint32_t, len: size_t) -> uint32_t {
    let numbers = unsafe {
        assert!(!n.is_null());
        slice::from_raw_parts(n, len as usize)
    };

    let sum = numbers.iter()
                     .filter(|&v| v % 2 == 0)
                     .fold(0, |acc, v| acc + v);
    sum as uint32_t
}

// ============ Tuples ============
fn flip_things_around_rust(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b, a)
}

#[repr(C)]
pub struct Tuple {
    x: uint32_t,
    y: uint32_t
}

impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Tuple {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> (u32, u32) {
        (tup.x, tup.y)
    }
}

#[no_mangle]
pub extern fn flip_things_around(tup: Tuple) -> Tuple {
    flip_things_around_rust(tup.into()).into()
}

// ============ Objects ============
pub struct ZipCodeDatabase {
    population: HashMap<String, u32>
}

impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new()
        }
    }
    fn populate(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }
    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

#[no_mangle]
pub extern fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.populate();
}

#[no_mangle]
pub extern fn zip_code_database_population_of(ptr: *const ZipCodeDatabase, zip: *const c_char) -> uint32_t {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.population_of(zip_str)
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }
