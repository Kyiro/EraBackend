use actix_web::{HttpResponse, Responder, get, post};
use serde_json::json;

#[get("/api/v2/versioncheck/{i}")]
pub async fn version_check() -> impl Responder {
    HttpResponse::NoContent()
    .json(json!({
        "type": "NO_UPDATE"
    }))
}

#[get("/api/game/v2/enabled_features")]
pub async fn enabled_features() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/receipts/v1/account/{i}/receipts")]
pub async fn receipts() -> impl Responder {
    HttpResponse::Ok()
    .json(Vec::<i8>::new())
}

#[get("/api/storefront/v2/catalog")]
pub async fn catalog() -> impl Responder {
    HttpResponse::Ok()
    .json(json!({
        "dailyPurchaseHrs": 24,
        "expiration": "6104-07-28T13:21:45Z",
        "refreshIntervalHrs": 1,
        "storefronts": Vec::<i8>::new()
    }))
}

#[get("/api/calendar/v1/timeline")]
pub async fn timeline() -> impl Responder {
    HttpResponse::NoContent()
}

#[post("/api/game/v2/tryPlayOnPlatform/account/{i}")]
pub async fn play_on_platform() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[get("/api/matchmaking/session/findPlayer/{i}")]
pub async fn find_player() -> impl Responder {
    HttpResponse::Ok()
}