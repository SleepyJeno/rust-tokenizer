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
pub async fn establish_connection(credentials: Credentials) -> Result<Client, Error>{
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