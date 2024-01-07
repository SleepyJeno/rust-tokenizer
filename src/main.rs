use block_modes::BlockMode;
use crate::{cipher::Cipher, db::{connection::{Credentials, establish_connection}, models::create_tables}};
mod cipher;

mod db;

fn main() {
    let cipher = Cipher::new();
    let token = tokenize("blah", &cipher);
    let (key, iv) = cipher.get_cipher_data();
    println!("cipher key {:?}, iv {:?}", key, iv);
    println!("tokenized input - {:?}", &token);
    println!("{:?}", detokenize(&token, &cipher));


    let credentials = Credentials {
        username: std::env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
        password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        host: std::env::var("DB_HOST").expect("DB_HOST must be set"),
        port: std::env::var("DB_PORT").expect("DB_PORT must be set"),
        db_name: std::env::var("DB_NAME").expect("DB_NAME must be set"),
        ssl_mode: std::env::var("DB_SSL_MODE").unwrap_or(String::from("disable"))
    };

    let mut client = establish_connection(credentials);
    create_tables(&mut client).expect("Error creating DB tables");
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