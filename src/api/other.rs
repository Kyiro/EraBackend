use actix_web::{get, post, HttpResponse, Responder};
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

#[get("/friends/api/public/friends/{i}")]
pub async fn friends() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/list/fortnite/{i}/recentPlayers")]
pub async fn recent_players() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/blocklist/{i}")]
pub async fn blocklist() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "blockedUsers": []
    }))
}

#[post("/datarouter/api/v1/public/data")]
pub async fn datarouter() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/eulatracking/api/shared/agreements/fn")]
pub async fn eulatracking() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}