use block_modes::BlockMode;
use crate::cipher::Cipher;
mod cipher;
mod db;

fn main() {
    let cipher = Cipher::new();
    let token = tokenize("blah", &cipher);
    let (key, iv) = cipher.get_cipher_data();
    //println!("cipher key {:?}, iv {:?}", key, iv);
    println!("tokenized input - {:?}", &token);
    println!("{:?}", detokenize(&token, &cipher));
}

fn tokenize(input: &str, cipher: &Cipher) -> String {
    //TODO: add check for input string size to be less than 128 bytes
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
