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

fn generate_salt() -> Vec<char> {
    let salt: Vec<char> = rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    salt
}

fn tokenize(input: &str) -> Vec<char> {
    let mut salt = generate_salt();
    let mut input_vec: Vec<char> = input.chars().collect();
    input_vec.extend(salt.iter()); //we want to save the salt value hence extend vs append
    // println!("{:?}, {:?}", input_vec, salt);

    let cipher = Aes128Cbc::new_from_slices(&salt, &iv).unwrap();


    let pos = input.len();

    let mut buffer = [0u8; 128];

    buffer[..pos].copy_from_slice(input);

    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    println!("\nCiphertext: {:?}",hex::encode(ciphertext));

    vec![char::from(2)]

}