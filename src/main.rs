use rand::RngCore;
use rand::{Rng, distributions::Alphanumeric};
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    println!("Hello, world!");
    tokenize("blah");
}

fn generate_key() -> Vec<u8> {
    let mut bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut bytes);

    bytes.to_vec()
}

fn generate_iv() -> Vec<u8> {
    let mut bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut bytes);

    bytes.to_vec()
}

fn tokenize(input: &str) -> String {
    let key = generate_key();
    let iv = generate_iv();
    let plaintext = input.as_bytes();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    hex::encode(ciphertext)
}