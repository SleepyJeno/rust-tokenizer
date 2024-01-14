use tokio_postgres::{NoTls, Error, Client};
use tokio;

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
    pub ssl_mode: String, //supported options are require, allow, prefer, verify-ca, verify-full
}

//TODO: add tls mode support
pub async fn establish_connection() -> Result<Client, Error>{

    let credentials = Credentials {
        username: std::env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
        password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        host: std::env::var("DB_HOST").expect("DB_HOST must be set"),
        port: std::env::var("DB_PORT").expect("DB_PORT must be set"),
        db_name: std::env::var("DB_NAME").expect("DB_NAME must be set"),
        ssl_mode: std::env::var("DB_SSL_MODE").unwrap_or(String::from("disable"))
    };

    let (client, connection) = tokio_postgres::connect(
                                    format!("host={} port ={} user={} password={} dbname={} sslmode={}",
                                    credentials.host, credentials.port, credentials.username, credentials.password, credentials.db_name, credentials.ssl_mode).as_str(),
                                    NoTls)
                                    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Connected to database");
    Ok(client)
}