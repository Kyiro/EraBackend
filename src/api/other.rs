use actix_web::{get, post, HttpResponse, Responder, web};
use crate::structs::app;
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

#[get("/friends/api/v1/{i}/settings")]
pub async fn settings() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[post("/datarouter/api/v1/public/data")]
pub async fn datarouter() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/eulatracking/api/shared/agreements/fn")]
pub async fn eulatracking() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/api/v1/events/Fortnite/download/{id}")]
pub async fn events(app: web::Data<app::State>) -> impl Responder {
    HttpResponse::Ok().json(app.events.clone())
}

#[get("/api/v1/events/Fortnite/{tournament}/history/{id}")]
pub async fn events_history(
    path: web::Path<(String, String)>
) -> impl Responder {
    let (tournament, id) = path.into_inner();
    
    HttpResponse::Ok()
        .json(json!([
            {
                "scoreKey": {
                    "gameId": "Fortnite",
                    "eventId": tournament,
                    "eventWindowId": "",
                    "_scoreId": null
                },
                "teamId": id,
                "teamAccountIds": [
                    id
                ],
                "liveSessionId": null,
                "pointsEarned": 999999,
                "score": 999999,
                "rank": 1,
                "percentile": 0.01,
                "pointBreakdown": {},
                "sessionHistory": []
            }
        ]))
}

#[get("/api/v1/events/Fortnite/{tournament}/{window}/history/{id}")]
pub async fn events_history_window(
    path: web::Path<(String, String, String)>
) -> impl Responder {
    let (tournament, window, id) = path.into_inner();
    
    HttpResponse::Ok()
        .json(json!([
            {
                "scoreKey": {
                    "gameId": "Fortnite",
                    "eventId": tournament,
                    "eventWindowId": window,
                    "_scoreId": null
                },
                "teamId": id,
                "teamAccountIds": [
                    id
                ],
                "liveSessionId": null,
                "pointsEarned": 999999,
                "score": 999999,
                "rank": 1,
                "percentile": 0.01,
                "pointBreakdown": {},
                "sessionHistory": []
            }
        ]))
}