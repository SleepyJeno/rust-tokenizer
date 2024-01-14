mod cipher;
mod db;
mod tokenization;
mod handlers;

use actix_web::HttpServer;
use db::{connection, models};
use cipher::Cipher;
use handlers::{tokenize};
use actix_web::{get, App, HttpResponse, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    // let credentials = connection::Credentials {
    //     username: std::env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
    //     password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
    //     host: std::env::var("DB_HOST").expect("DB_HOST must be set"),
    //     port: std::env::var("DB_PORT").expect("DB_PORT must be set"),
    //     db_name: std::env::var("DB_NAME").expect("DB_NAME must be set"),
    //     ssl_mode: std::env::var("DB_SSL_MODE").unwrap_or(String::from("disable"))
    // };

    // let mut client = connection::establish_connection(credentials);
    // let tables_exist =  models::check_tables_exist(&mut client).unwrap();
    // if !tables_exist {
    //     println!("No DB tables found - creating DB tables");
    //     models::create_tables(&mut client).expect("Error creating DB tables");
    // }


    // let cipher = Cipher::new();
    // let token = tokenization::tokenize_str("abcd1234", &cipher, &mut client);
    // println!("tokenized input - {:?}", &token);
    // println!("detokenized: {:?}", tokenization::detokenize_str(&token, &mut client));

    HttpServer::new(|| {
        App::new()
            .service(tokenize)
            //.service(web::resource("/tokenize").route(web::post().to(tokenize)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
