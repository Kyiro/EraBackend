use crate::structs::account::*;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{prelude::*, Duration};
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

#[get("/api/oauth/verify")]
pub async fn oauth_verify(req: HttpRequest) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(data) => data.to_str().unwrap().replace("bearer ", ""),
        None => return HttpResponse::Unauthorized().into()
    };
    
    HttpResponse::Ok().json(json!({
        "token": token,
        "token_type": "bearer",
        "client_id": "3446cd72694c4a4485d81b77adbb2141",
        "internal_client": true,
        "client_service": "fortnite",
        "expires_in": 2147483647,
        "expires_at": (Utc::now() + Duration::minutes(2147483647))
        .to_rfc3339_opts(SecondsFormat::Secs, true),
        "app": "fortnite",
        "scope": []
    }))
}

#[get("/api/public/account/{id}")]
pub async fn personal_account(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
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

#[derive(Deserialize)]
pub struct PublicAccount {
    #[serde(rename = "accountId")]
    account_id: String,
}

#[get("/api/public/account")]
pub async fn personal_account_query(query: web::Query<PublicAccount>) -> impl Responder {
    let query = query.into_inner();
    HttpResponse::Ok().json(json!([{
        "id": query.account_id,
        "displayName": "Project Era",
        "externalAuths": {}
    }]))
}

#[get("/api/public/account/{i}/externalAuths")]
pub async fn external_auths() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/accounts/{i}/metadata")]
pub async fn accounts_metadata() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[delete("/api/oauth/sessions/kill")]
pub async fn kill_sessions() -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/api/oauth/sessions/kill/{i}")]
pub async fn kill_sessions_id() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/api/epicdomains/ssodomains")]
pub async fn ssodomains() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}