use postgres::{Client, NoTls};

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
    let client = Client::connect(
                                    format!("host={} port ={} user={} password={} dbname={} sslmode={}",
                                    credentials.host, credentials.port, credentials.username, credentials.password, credentials.db_name, credentials.ssl_mode).as_str(),
                                    NoTls)
                                    .expect("Failed to connect to database"
                                    );
    println!("Connected to database");
    client
}