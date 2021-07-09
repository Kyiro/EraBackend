use actix_web::{HttpRequest, HttpResponse, Responder, get, post};
use chrono::prelude::*;
use regex::Regex;
use serde_json::json;

#[get("/api/v2/versioncheck/{i}")]
pub async fn version_check() -> impl Responder {
    HttpResponse::NoContent().json(json!({
        "type": "NO_UPDATE"
    }))
}

#[get("/api/game/v2/enabled_features")]
pub async fn enabled_features() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/receipts/v1/account/{i}/receipts")]
pub async fn receipts() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/storefront/v2/catalog")]
pub async fn catalog() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "dailyPurchaseHrs": 24,
        "expiration": "6104-07-28T13:21:45Z",
        "refreshIntervalHrs": 1,
        "storefronts": Vec::<i8>::new()
    }))
}

#[post("/api/game/v2/tryPlayOnPlatform/account/{i}")]
pub async fn play_on_platform() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[get("/api/matchmaking/session/findPlayer/{i}")]
pub async fn find_player() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/api/game/v2/world/info")]
pub async fn world_info() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "theaters": Vec::<i8>::new(),
        "missions": Vec::<i8>::new(),
        "missionAlerts": Vec::<i8>::new()
    }))
}

// bad code but it works lol
pub fn get_season(useragent: &str) -> Option<&str> {
    let regex = match Regex::new(r"\+\+Fortnite\+Release-(\d+)\.(\d+).*-CL") {
        Ok(data) => data,
        Err(_) => return None
    };
    // lowkey spaghetti
    let capture = match regex.captures(useragent) {
        Some(data) => match data.get(1) {
            Some(data) => data,
            None => return None
        },
        None => return None
    };
    
    Some(capture.as_str())
}

#[get("/api/calendar/v1/timeline")]
pub async fn timeline(req: HttpRequest) -> impl Responder {
    // lol
    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let season = get_season(useragent).unwrap_or("2");
    
    HttpResponse::Ok().json(json!({
      "channels": {
        "client-events": {
          "states": [
            {
              "validFrom": "2000-01-01T10:00:00Z",
              "activeEvents": [
                {
                  "eventType": format!("EventFlag.Season{}", season.clone()),
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                },
                {
                  "eventType": match season {
                      "1" | "2" => String::from("EventFlag.LobbyWinterDecor"),
                      _ => format!("EventFlag.LobbySeason{}", season.clone())
                  },
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                }
              ],
              "state": {
                "activeStorefronts": [],
                "eventNamedWeights": {},
                "seasonNumber": season.parse::<i32>().unwrap(),
                "seasonTemplateId": format!("AthenaSeason:athenaseason{}", season),
                "matchXpBonusPoints": 0,
                "seasonBegin": "2000-01-01T10:00:00Z",
                "seasonEnd": "9999-01-01T14:00:00Z",
                "seasonDisplayedEnd": "9999-01-01T07:30:00Z",
                "weeklyStoreEnd": "9999-01-01T00:00:00Z",
                "stwEventStoreEnd": "9999-01-01T00:00:00.000Z",
                "stwWeeklyStoreEnd": "9999-01-01T00:00:00.000Z",
                "dailyStoreEnd": "9999-01-01T00:00:00Z"
              }
            }
          ],
          "cacheExpire": "9999-01-01T22:28:47.830Z"
        }
      },
      "eventsTimeOffsetHrs": 0,
      "cacheIntervalMins": 9999,
      "currentTime": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
    }))
}

#[get("/api/storefront/v2/keychain")]
pub async fn keychain() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "https://api.nitestats.com/v1/epic/keychain"))
        .finish()
}
