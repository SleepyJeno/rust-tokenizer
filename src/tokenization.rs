use crate::cipher::Cipher;
use crate::db::queries;

pub async fn tokenize_str(input: &str) -> String {
    //TODO: add check for input string size to be less than 128 bytes
    let cipher = Cipher::new();
    let plaintext = input.as_bytes();
    let ciphertext = cipher.encrypt(&plaintext);
    let tokenized_string = hex::encode(ciphertext);
    let (key, iv) = cipher.get_cipher_data();
    queries::write_tokenized_data(&tokenized_string, &key, &iv).await.ok();
    tokenized_string
}

pub async fn detokenize_str(tokenized_string: &String) -> String {
    let (tokenized_string_result, cipher_key_result, cipher_iv_result) =
        match queries::read_tokenized_data(&tokenized_string).await {
            Ok((tokenized_string, cipher_key, cipher_iv)) => (tokenized_string, cipher_key, cipher_iv),
            Err(e) => {
                return e.to_string();
            }
        };
    let cipher = Cipher::from(cipher_key_result, cipher_iv_result);
    let ciphertext = hex::decode(tokenized_string_result).unwrap();
    let buffer = ciphertext.to_vec();
    let decrypted = cipher.decrypt(&buffer);
    String::from_utf8(decrypted.to_vec()).unwrap()
}