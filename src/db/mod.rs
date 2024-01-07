use postgres::{Client, NoTls, Error};

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub url: String,
    pub port: String,
    pub db_name: String,
}

pub fn establish_connection(credentials: Credentials) {
    let mut client = Client::connect(format!("postgresql://{}:{}@{}:{}/{}", credentials.username, credentials.password, credentials.url, credentials.port, credentials.db_name).as_str(), NoTls).
                                     expect("Failed to connect to database");
    println!("Connected to database");
}