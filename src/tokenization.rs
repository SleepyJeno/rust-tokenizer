use tokio_postgres::Client;
use crate::cipher::Cipher;
use crate::db;

pub async fn tokenize_str(input: &str) -> String {
    //TODO: add check for input string size to be less than 128 bytes
    let cipher = Cipher::new();
    let plaintext = input.as_bytes();
    let ciphertext = cipher.encrypt(&plaintext);
    let tokenized_string = hex::encode(ciphertext);
    let (key, iv) = cipher.get_cipher_data();
    db::queries::write_tokenized_data(&tokenized_string, &key, &iv).await.ok();
    tokenized_string
}

// pub async fn detokenize_str(tokenized_string: &String, client: &mut Client) -> String {
//     let Ok((tokenised_string, key, iv)) = db::queries::read_tokenized_data(client, tokenized_string).await;
//     let cipher = Cipher::from(key, iv);
//     let ciphertext = hex::decode(tokenised_string).unwrap();
//     let buffer = ciphertext.to_vec();
//     let decrypted = cipher.decrypt(&buffer);
//     String::from_utf8(decrypted.to_vec()).unwrap()
// }