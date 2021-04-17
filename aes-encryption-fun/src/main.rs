use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

fn u8ToString(buf:Vec<u8>) -> String{
    let s = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    return s;
}

fn toBase64String(data:Vec<u8>) -> String{
    return base64::encode(&data);
}

fn main() {
    // create an alias for convenience
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    let key = hex!("000102030405060708090a0b0c0d0e0f");
    println!("Key: {:?}",toBase64String(key.to_vec()));
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    println!("IV: {:?}",iv.to_vec());
    let plaintext = b"Hello world!";
    println!("Plaintext: {:?}",u8ToString(plaintext.to_vec()));
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();

    // buffer must have enough space for message+padding
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    println!("Ciphertext: {:?}",toBase64String(ciphertext.to_vec()));

    assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));

    // re-create cipher mode instance and decrypt the message
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
    assert_eq!(decrypted_ciphertext, plaintext);
}
