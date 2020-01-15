extern crate boxing_arena;
use boxing_arena::BoxingArena;

fn main() {
  // Prepare an long-lived arena:
  let mut ba = BoxingArena::new();
  print_capacity(&ba);

  // ... per allocation ... :
  // In place of using `Box::new` directly, we do:
  let boxed_big_value = ba.rebox("This is a big value");
  println!("Original: {:?}",boxed_big_value);
  print_capacity(&ba);

  // Instead of letting Rust drop and deallocate the Box, we do:
  let r = ba.unbox(boxed_big_value);	
  println!("Original: {:?}",r);
  print_capacity(&ba);
}

fn print_capacity(boxa:&boxing_arena::BoxingArena<&str>){
  println!("Capacity {:?}",boxa.capacity());
}
