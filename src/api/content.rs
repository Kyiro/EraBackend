use crate::structs::app::State;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/pages/fortnite-game")]
pub async fn fortnite_game(app: web::Data<State>) -> impl Responder {
    HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .body(app.game.clone())
}
