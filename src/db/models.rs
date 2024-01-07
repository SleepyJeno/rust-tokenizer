use postgres::{Client, Error};

//TODO replace raw sql with ORM i.e. Diesel
pub fn create_tables(client: &mut Client) -> Result<(), Error> {
    let tokens_table = "CREATE TABLE IF NOT EXISTS tokens (
                            token_id SERIAL PRIMARY KEY,
                            tokenized_string VARCHAR(255) NOT NULL,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                            )";
    client.batch_execute(tokens_table)?;

    let keys_table = "CREATE TABLE IF NOT EXISTS keys (
                            key_id SERIAL PRIMARY KEY,
                            token_id INT REFERENCES tokens(token_id) ON DELETE CASCADE,
                            cipher_key BYTEA NOT NULL,
                            iv BYTEA NOT NULL,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                            )";
    client.batch_execute(keys_table)?;

    Ok(())
}

//TODO:
//- add function to check if tables exist
//- add function to check if table schema matches