use crate::structs::account::*;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OAuthToken {
    pub grant_type: String,
}

#[post("/api/oauth/token")]
pub async fn oauth_token(body: web::Form<OAuthToken>, req: HttpRequest) -> impl Responder {
    let basic = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    match body.grant_type.as_str() {
        "client_credentials" => HttpResponse::Ok().json(ClientCreds::new(basic)),
        _ => HttpResponse::Ok().json(BearerToken::new(basic)),
    }
}

#[post("/api/oauth/verify")]
pub async fn oauth_verify() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/api/public/account/{id}")]
pub async fn personal_account(web::Path(id): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "id": id,
        "displayName": "Project Era",
        "name": "Project",
        "email": "era@erafn.glitch.me",
        "failedLoginAttempts": 0,
        // Project Era Discord Server creation date
        "lastFailedLogin": "2021-01-22T23:00:00.000Z",
        "lastLogin": "2021-01-22T23:00:00.000Z",
        "numberOfDisplayNameChanges": 1,
        "ageGroup": "UNKNOWN",
        "headless": false,
        // funny
        "country": "PL",
        "lastName": "Era",
        "preferredLanguage": "en",
        "lastDisplayNameChange": "2021-01-22T23:00:00.000Z",
        "canUpdateDisplayName": true,
        "tfaEnabled": false,
        "emailVerified": true,
        "minorVerified": false,
        "minorExpected": false,
        "minorStatus": "UNKNOWN"
    }))
}

#[get("/api/public/account/{i}/externalAuths")]
pub async fn external_auths() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[delete("/api/oauth/sessions/kill")]
pub async fn kill_sessions() -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/api/oauth/sessions/kill/{i}")]
pub async fn kill_sessions_id() -> impl Responder {
    HttpResponse::NoContent()
}
