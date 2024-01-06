use rand::RngCore;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;


struct Cipher {
    key: Vec<u8>,
    iv: Vec<u8>,
    cipher: Aes128Cbc
}

impl Cipher {
    fn new() -> Self{
        let key = Self::generate_key();
        let iv = Self::generate_iv();
        //Aes128Cbc::new_from_slices(&key, &iv).unwrap()
        let cipher: Cbc<Aes128, Pkcs7> = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

        Self {
            key,
            iv,
            cipher
          }
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
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", tokenize("blah"));
}

fn tokenize(input: &str) -> String {
    let cipher_base = Cipher::new();
    let plaintext = input.as_bytes();
    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher_base.cipher.encrypt(&mut buffer, pos).unwrap();

    hex::encode(ciphertext)
}

fn detokenize(input: &str, cipher: Cbc<Aes128, Pkcs7>) -> String {

    String::from("a")
}