fn main() {
    const fn type_size<T>() -> usize {
        std::mem::size_of::<T>()
    }

    const _: () = assert!(type_size::<u32>() == 4);
    const _: () = assert!(type_size::<u64>() == 8);

    println!("All static assertions passed!");./r   
}