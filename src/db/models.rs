use tokio_postgres::{Client, Error};

//TODO replace raw sql with ORM i.e. Diesel
pub async fn create_tables(client: &mut Client) -> Result<(), Error> {
    let tokens_table = "CREATE TABLE IF NOT EXISTS tokens (
                            token_id SERIAL PRIMARY KEY,
                            tokenized_string VARCHAR(255) NOT NULL,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                            );
                            CREATE INDEX idx_tokenized_string ON Tokens (tokenized_string);";
    client.batch_execute(tokens_table).await?;

    let keys_table = "CREATE TABLE IF NOT EXISTS keys (
                            key_id SERIAL PRIMARY KEY,
                            token_id INT REFERENCES tokens(token_id) ON DELETE CASCADE,
                            cipher_key BYTEA NOT NULL,
                            iv BYTEA NOT NULL,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                            )";
    client.batch_execute(keys_table).await?;

    Ok(())
}

pub async fn check_tables_exist(client: &mut Client) -> Result<bool, Error> {
    let tokens_table = "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'tokens');";
    let keys_table = "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'keys');";

    let tokens_exist = client.query_one(tokens_table, &[]).await?.get::<_, bool>(0);
    let keys_exist = client.query_one(keys_table, &[]).await?.get::<_, bool>(0);

    Ok(tokens_exist && keys_exist)
}