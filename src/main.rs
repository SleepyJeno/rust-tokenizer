mod cipher;
mod db;
mod tokenization;
mod handlers;

use actix_web::HttpServer;
use db::{connection, models};
use handlers::{tokenize, detokenize};
use actix_web::App;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    
    //on startup check if DB tables exist, if not create them
    let mut client = connection::establish_connection().await.expect("Error connecting to DB");
    let tables_exist =  models::check_tables_exist(&mut client).await.unwrap();
    if !tables_exist {
        println!("No DB tables found - creating DB tables");
        models::create_tables(&mut client).await.unwrap()
    }

    HttpServer::new(|| {
        App::new()
            .service(tokenize)
            .service(detokenize)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
