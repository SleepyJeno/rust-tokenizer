use postgres::{Client, Error};
use chrono::{DateTime, Utc};


//TODO replace raw sql with ORM i.e. Diesel
pub fn write_tokenized_data(client: &mut Client, tokenized_string: &String) -> Result<(), Error> {
    let query = "INSERT INTO tokens (tokenized_string) VALUES ($1) RETURNING token_id, created_at";
    
    let row = client
        .query_one(query, &[&tokenized_string]).expect("Error writing the token to the DB");

    let token_id: i32 = row.get("token_id");
    //TODO fix retrieval of created_at timestamp
    // let created_at: DateTime<Utc> = row.get("created_at");
    // Ok((token_id, created_at))
    println!("token_id: {},", token_id);
    Ok(())
}