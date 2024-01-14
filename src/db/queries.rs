use tokio_postgres::{Client, Error};

//TODO replace raw sql with ORM i.e. Diesel
pub async fn write_tokenized_data(client: &mut Client, tokenized_string: &String, cipher_key: &Vec<u8>, cipher_iv: &Vec<u8>) -> Result<(), Error> {
    let tokens_query = "INSERT INTO tokens (tokenized_string) VALUES ($1) RETURNING token_id, created_at";
    let keys_query = "INSERT INTO Keys (token_id, cipher_key, iv) VALUES ($1, $2, $3) RETURNING key_id";

    let row: tokio_postgres::Row = client
        .query_one(tokens_query, &[&tokenized_string]).await?;

    let token_id: i32 = row.get("token_id");

    println!("Token inserted with ID: {}", token_id);

    let _key_row: tokio_postgres::Row = client.query_one(keys_query, &[&token_id, &cipher_key, &cipher_iv]).await?;
    //TODO fix retrieval of created_at timestamp
    // let created_at: DateTime<Utc> = row.get("created_at");
    // Ok((token_id, created_at))

    Ok(())
}

pub async fn read_tokenized_data(client: &mut Client, tokenized_string: &String) -> Result<(String, Vec<u8>, Vec<u8>), Error> {
    let tokens_query = "SELECT tokenized_string, cipher_key, iv FROM tokens INNER JOIN keys ON tokens.token_id = keys.token_id WHERE tokens.tokenized_string = $1";

    let row: tokio_postgres::Row = client
        .query_one(tokens_query, &[&tokenized_string]).await?;

    let tokenized_string: String = row.get("tokenized_string");
    let cipher_key: Vec<u8> = row.get("cipher_key");
    let cipher_iv: Vec<u8> = row.get("iv");

    Ok((tokenized_string, cipher_key, cipher_iv))
}