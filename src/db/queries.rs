use postgres::{Client, Error};

//TODO replace raw sql with ORM i.e. Diesel
pub fn write_tokenized_data(client: &mut Client, tokenized_string: &String, cipher_key: &Vec<u8>, cipher_iv: &Vec<u8>) -> Result<(), Error> {
    let tokens_query = "INSERT INTO tokens (tokenized_string) VALUES ($1) RETURNING token_id, created_at";
    let keys_query = "INSERT INTO Keys (token_id, cipher_key, iv) VALUES ($1, $2, $3) RETURNING key_id";

    let row: postgres::Row = client
        .query_one(tokens_query, &[&tokenized_string]).expect("Error writing the token to the DB");

    let token_id: i32 = row.get("token_id");

    println!("Token inserted with ID: {}", token_id);

    let _key_row: postgres::Row = client.query_one(keys_query, &[&token_id, &cipher_key, &cipher_iv]).expect("Error writing key data to the DB");
    //TODO fix retrieval of created_at timestamp
    // let created_at: DateTime<Utc> = row.get("created_at");
    // Ok((token_id, created_at))

    Ok(())
}

pub fn read_tokenized_data(client: &mut Client, tokenized_string: &String) -> Result<(String, Vec<u8>, Vec<u8>), Error> {
    let tokens_query = "SELECT tokenized_string, cipher_key, iv FROM tokens INNER JOIN keys ON tokens.token_id = keys.token_id WHERE tokens.tokenized_string = $1";

    let row: postgres::Row = client
        .query_one(tokens_query, &[&tokenized_string]).expect("Error reading the token from the DB");

    let tokenized_string: String = row.get("tokenized_string");
    let cipher_key: Vec<u8> = row.get("cipher_key");
    let cipher_iv: Vec<u8> = row.get("iv");

    Ok((tokenized_string, cipher_key, cipher_iv))
}