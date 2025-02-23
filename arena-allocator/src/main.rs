use std::cell::UnsafeCell;
use std::mem;
use std::ptr::NonNull;

struct Arena {
    buffer: UnsafeCell<Vec<u8>>,
    position: usize,
}

impl Arena {
    fn new(size: usize) -> Self {
        Arena {
            buffer: UnsafeCell::new(vec![0; size]),
            position: 0,
        }
    }

    fn alloc<T>(&mut self, value: T) -> Option<NonNull<T>> {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let buffer = unsafe { &mut *self.buffer.get() };

        // Align position
        self.position = (self.position + align - 1) & !(align - 1);

        if self.position + size > buffer.len() {
            return None;
        }

        let ptr = unsafe {
            buffer.as_mut_ptr().add(self.position) as *mut T
        };
        unsafe {
            ptr.write(value);
        }
        self.position += size;

        Some(unsafe { NonNull::new_unchecked(ptr) })
    }
}

fn main() {
    let mut arena = Arena::new(1024);
    let num = arena.alloc(42).unwrap();
    let string = arena.alloc(String::from("Hello")).unwrap();
    
    unsafe {
        println!("Number: {}", num.as_ref());
        println!("String: {}", string.as_ref());
    }
}