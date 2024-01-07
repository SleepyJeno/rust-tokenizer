use postgres::{Client, NoTls, Error};

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub ssl_mode: String, //supported options are require, allow, prefer, verify-ca, verify-full
}

//TODO: add tls mode support
pub fn establish_connection(credentials: Credentials) -> Client{
    let mut client = Client::connect(
                                    format!("host={} port ={} user={} password={} dbname={} sslmode={}",
                                    credentials.host, credentials.port, credentials.username, credentials.password, credentials.db_name, credentials.ssl_mode).as_str(),
                                    NoTls)
                                    .expect("Failed to connect to database"
                                    );
    println!("Connected to database");
    client
}

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
                            cipher_key integer[] NOT NULL,
                            iv integer[] NOT NULL,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                            )";
    client.batch_execute(keys_table)?;

    Ok(())
}