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
    
    //on startup check if DB tables exist, if not create them
    let mut client = connection::establish_connection().await.expect("Error connecting to DB");
    let tables_exist =  models::check_tables_exist(&mut client).await.unwrap();
    if !tables_exist {
        println!("No DB tables found - creating DB tables");
        models::create_tables(&mut client).await.unwrap()
    }


    // let cipher = Cipher::new();
    // let token = tokenization::tokenize_str("abcd1234", &cipher, &mut client);
    // println!("tokenized input - {:?}", &token);
    // println!("detokenized: {:?}", tokenization::detokenize_str(&token, &mut client));

    HttpServer::new(|| {
        App::new()
            .service(tokenize)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
