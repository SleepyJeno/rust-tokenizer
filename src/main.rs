use block_modes::BlockMode;
use crate::cipher::Cipher;
mod cipher;

fn main() {
    let cipher = Cipher::new();
    let token = tokenize("blah", &cipher);
    println!("tokenized input - {:?}", &token);
    println!("{:?}", detokenize(&token, &cipher));
}

fn tokenize(input: &str, cipher: &Cipher) -> String {
    let plaintext = input.as_bytes();
    let ciphertext = cipher.encrypt(&plaintext);
    hex::encode(ciphertext)
}

fn detokenize(input: &str, cipher: &Cipher) -> String {
    let ciphertext = hex::decode(input).unwrap();
    let mut buffer = ciphertext.to_vec();
    let decrypted = cipher.clone().decrypt(&buffer);
    String::from_utf8(decrypted.to_vec()).unwrap()
}