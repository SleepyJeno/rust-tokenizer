use postgres::Client;
use crate::{cipher::Cipher, db::{connection::{Credentials, establish_connection}, models::create_tables}};
mod cipher;

mod db;

fn main() {
    let credentials = Credentials {
        username: std::env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
        password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        host: std::env::var("DB_HOST").expect("DB_HOST must be set"),
        port: std::env::var("DB_PORT").expect("DB_PORT must be set"),
        db_name: std::env::var("DB_NAME").expect("DB_NAME must be set"),
        ssl_mode: std::env::var("DB_SSL_MODE").unwrap_or(String::from("disable"))
    };

    let mut client = establish_connection(credentials);
    let tables_exist =  db::models::check_tables_exist(&mut client).unwrap();
    if !tables_exist {
        println!("No DB tables found - creating DB tables");
        create_tables(&mut client).expect("Error creating DB tables");
    }


    let cipher = Cipher::new();
    let token = tokenize("abcd1234", &cipher, &mut client);
    println!("tokenized input - {:?}", &token);
    println!("detokenized: {:?}", detokenize(&token, &mut client));
}

fn tokenize(input: &str, cipher: &Cipher, client: &mut Client) -> String {
    //TODO: add check for input string size to be less than 128 bytes
    let plaintext = input.as_bytes();
    let ciphertext = cipher.encrypt(&plaintext);
    let tokenized_string = hex::encode(ciphertext);
    let (key, iv) = cipher.get_cipher_data();
    db::queries::write_tokenized_data(client, &tokenized_string, &key, &iv).ok();
    tokenized_string
}

fn detokenize(tokenized_string: &String, client: &mut Client) -> String {
    let (tokenised_string, key, iv) = db::queries::read_tokenized_data(client, tokenized_string).unwrap();
    let cipher = Cipher::from(key, iv);
    let ciphertext = hex::decode(tokenised_string).unwrap();
    let mut buffer = ciphertext.to_vec();
    let decrypted = cipher.decrypt(&buffer);
    String::from_utf8(decrypted.to_vec()).unwrap()
}