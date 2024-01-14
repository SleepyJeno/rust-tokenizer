use actix_web::{self, HttpResponse, Responder, web};
use crate::tokenization;
use crate::db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenizeRequest {
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenizeResponse {
    pub result: String,
}

// Implement Default for TokenizeResponse
// This allows Actix to automatically generate responses with a default value
impl Default for TokenizeResponse {
    fn default() -> Self {
        TokenizeResponse {
            result: String::new(),
        }
    }
}

#[actix_web::post("/tokenize")]
async fn tokenize(data: web::Json<TokenizeRequest>) -> impl Responder  {
    let input = &data.input;

    let result = tokenization::tokenize_str(input).await;
    let response: TokenizeResponse = TokenizeResponse { result };
    HttpResponse::Ok().json(response)
}


