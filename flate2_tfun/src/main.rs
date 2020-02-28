use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::io;

fn main() {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = e.write_all(b"foo");
    let _ = e.write_all(b"bar");
    let compressed_bytes = e.finish().unwrap();
    println!("Compressed Bytes {:?}", compressed_bytes);

    let decoded = decode_reader(compressed_bytes);
    println!("Decoded {:?}", decoded);
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s)?;
    Ok(s)
}