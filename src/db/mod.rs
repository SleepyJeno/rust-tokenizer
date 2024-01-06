use postgres::{Client, NoTls, Error};

struct Credentials {
    username: String,
    password: String,
    url: String,
}

pub fn establish_connection() {
    let credentials = Credentials {
        username: std::env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
        password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        url: std::env::var("DB_URL").expect("DB_URL must be set")
    };
    let mut client = Client::connect(format!("postgresql://{}:{}@{}/library", credentials.username, credentials.password, credentials.url).as_str(), NoTls).unwrap();

}