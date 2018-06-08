#![feature(inclusive_range_syntax)]
#![feature(i128_type)]
// extern crate num_bigint;
// use num_bigint::BigUint;

use std::ffi::{CString};
use std::os::raw::{c_char};

#[no_mangle]
pub extern "C" fn pascal(num_of_lines: u32) -> *mut c_char {
    let num_of_lines = num_of_lines as usize;
    let mut prev_line: Vec<u128> = vec!();
    prev_line.push(1);
    for i in 1 ..= num_of_lines {
        let mut line: Vec<u128> = vec!();
        line.push(1);
        for j in 1 .. i {
            line.push(prev_line.get(j-1).unwrap() + prev_line.get(j).unwrap());
        }
        line.push(1);
        prev_line = line.clone();
    }
    format!("{:?}", prev_line);
    return CString::new("test").unwrap().into_raw();
}

fn main() {}
