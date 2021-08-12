use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/waitingroom/api/waitingroom")]
pub async fn waitingroom() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/party/api/v1/Fortnite/user/{u}")]
pub async fn party_user() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "current": [],
        "pending": [],
        "invites": [],
        "pings": []
    }))
}
