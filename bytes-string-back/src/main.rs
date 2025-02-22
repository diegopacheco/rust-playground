fn main() {
    // String to bytes
    let message = String::from("Hello, World!");
    let bytes = message.as_bytes();
    println!("String as bytes: {:?}", bytes);

    // Bytes to string
    let string_from_bytes = String::from_utf8(bytes.to_vec())
        .expect("Failed to convert bytes to string");
    println!("Bytes back to string: {}", string_from_bytes);

    // Alternative way using str::from_utf8
    let str_from_bytes = std::str::from_utf8(bytes)
        .expect("Failed to convert bytes to str");
    println!("Bytes to str: {}", str_from_bytes);

    // Working with raw bytes
    let raw_bytes = vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]; // "Hello World" in ASCII
    let string_from_raw = String::from_utf8(raw_bytes)
        .expect("Failed to convert raw bytes to string");
    println!("Raw bytes to string: {}", string_from_raw);
}