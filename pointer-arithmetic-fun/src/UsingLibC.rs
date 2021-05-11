extern crate libc;

use std::mem::size_of;
use std::ptr::write;
use std::ptr::read;
use std::mem::transmute;

use libc::calloc;
use libc::free;
use libc::c_void;

pub struct Array {
    length: usize,
    data: *mut f64,
}

impl Array {
    pub fn new(length: usize) -> Array {
        unsafe {
            Array {
                length: length,
                data: calloc(size_of::<f64>(), length) as *mut f64,
            }
        }
    }

    pub fn set(&mut self, offset: usize, value: f64) {
        if offset < self.length {
            unsafe {
                let root: *mut f64 = transmute(transmute::<*mut f64, u64>(self.data) +
                    (size_of::<f64>() * offset) as u64);
                println!("Write: [{:?}] -> {}", root, value);
                write(root, value);
            }
        } else {
            println!("Write: Nope: [{}] is out of bounds", offset);
        }
    }

    pub fn get(&self, offset: usize) -> f64 {
        if offset < self.length {
            unsafe {
                let root: *const f64 = transmute(transmute::<*mut f64, u64>(self.data) +
                    (size_of::<f64>() * offset) as u64);
                let rtn = read::<f64>(root);
                println!("Read: [{:?}] -> {}", root, rtn);
                return rtn;
            }
        }
        println!("Read: Nope: [{}] is out of bounds", offset);
        0.0
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            free(self.data as *mut c_void);
        }
    }
}

pub fn compute(){
    let mut tmp = Array::new(4);
    tmp.set(0, 100.5);
    tmp.set(1, 101.5);
    tmp.set(2, 102.5);
    tmp.set(3, 103.5);
    tmp.set(4, 104.5);
    tmp.get(0);
    tmp.get(1);
    tmp.get(2);
    tmp.get(3);
    tmp.get(4);
}