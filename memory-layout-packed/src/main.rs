fn main() {
    #[repr(packed(2))]
    struct Packed {
        byte: u8,
        word: u16,
        dword: u32,
    }

    let packed = Packed {
        byte: 1,
        word: 2,
        dword: 3,
    };

    println!("Size: {}", std::mem::size_of::<Packed>());
    println!("Alignment: {}", std::mem::align_of::<Packed>());
}