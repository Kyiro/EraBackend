use crate::utils::get_build;
use crate::{structs::app::State, utils::Build};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{prelude::*, Duration};
use serde_json::{json, Value};

#[get("/api/v2/versioncheck/{i}")]
pub async fn version_check_v2() -> impl Responder {
    HttpResponse::NoContent().json(json!({
        "type": "NO_UPDATE"
    }))
}

#[get("/api/versioncheck")]
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
        "expiration": "9999-01-01T22:28:47.830Z",
        "refreshIntervalHrs": 24,
        "storefronts": [
            {
                "name": "BRSeasonalStorefront",
                "catalogEntries": [
                    item(None, "AthenaCharacter:CID_015_Athena_Commando_F", 800),
                    item(None, "AthenaPickaxe:HalloweenScythe", 800),
                    item(None, "AthenaCharacter:CID_010_Athena_Commando_M", 800),
                    item(None, "AthenaCharacter:CID_012_Athena_Commando_M", 800),
                ]
            },
            {
                "name": "BRDailyStorefront",
                "catalogEntries": [
                    item(None, "AthenaCharacter:CID_015_Athena_Commando_F", 800),
                    item(None, "AthenaPickaxe:HalloweenScythe", 800),
                    item(None, "AthenaCharacter:CID_010_Athena_Commando_M", 800),
                    item(None, "AthenaCharacter:CID_012_Athena_Commando_M", 800),
                ]
            },
            {
                "name": "BRWeeklyStorefront",
                "catalogEntries": [
                    item(Some("DA_Featured_SFemaleHalloween"), "AthenaCharacter:CID_029_Athena_Commando_F_Halloween", 1500),
                    item(Some("DA_Featured_SMaleHalloween"), "AthenaCharacter:CID_030_Athena_Commando_M_Halloween", 1200)
                ]
            }
        ]
    }))
}

pub fn item(da: Option<&str>, id: &str, price: i32) -> Value {
    json!({
        "devName":  id,
        "offerId": "v2:/erabackend",
        "fulfillmentIds": [],
        "dailyLimit": -1,
        "weeklyLimit": -1,
        "monthlyLimit": -1,
        "categories": [],
        "prices": [
            {
                "currencyType": "MtxCurrency",
                "currencySubType": "",
                "regularPrice": price,
                "finalPrice": price,
                "saleExpiration": "9999-12-31T23:59:59.999Z",
                "basePrice": price
            }
        ],
        "matchFilter": "",
        "filterWeight": 0,
        "appStoreId": [],
        "requirements": [
            {
                "requirementType": "DenyOnItemOwnership",
                "requiredId":  id,
                "minQuantity": 1
            }
        ],
        "offerType": "StaticPrice",
        "giftInfo": {
            "bIsEnabled": false,
            "forcedGiftBoxTemplateId": "",
            "purchaseRequirements": [],
            "giftRecordIds": []
        },
        "refundable": true,
        "metaInfo": [],
        "displayAssetPath": if let Some(da) = da {
            "/Game/Catalog/DisplayAssets/".to_owned() + da + "." + da
        } else { String::new() },
        "itemGrants": [
            {
                "templateId":  id,
                "quantity": 1
            }
        ],
        "sortPriority": 0,
        "catalogGroupPriority": 0
    })
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

#[get("/api/calendar/v1/timeline")]
pub async fn timeline(req: HttpRequest) -> impl Responder {
    // lol
    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let build = get_build(useragent).unwrap_or(Build::default());
    let day = (Utc::now() + Duration::days(1)).to_rfc3339_opts(SecondsFormat::Secs, true);

    HttpResponse::Ok().json(json!({
      "channels": {
        "client-events": {
          "states": [
            {
              "validFrom": "2000-01-01T10:00:00Z",
              "activeEvents": [
                {
                  "eventType": format!("EventFlag.Season{}", build.season),
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                },
                {
                  "eventType": match build.season {
                      1 | 2 => String::from("EventFlag.LobbyWinterDecor"),
                      _ => if build.patch == Some(6.21) {
                        String::from("EventFlag.LobbySeason6Halloween")
                      } else {
                          format!("EventFlag.LobbySeason{}", build.season)
                        }
                  },
                  "activeUntil": "9999-01-01T22:28:47.830Z",
                  "activeSince": "2000-01-01T10:00:00Z"
                }
              ],
              "state": {
                "activeStorefronts": [],
                "eventNamedWeights": {},
                "seasonNumber": build.season,
                "seasonTemplateId": format!("AthenaSeason:athenaseason{}", build.season),
                "matchXpBonusPoints": 0,
                "seasonBegin": "2000-01-01T10:00:00Z",
                "seasonEnd": day,
                "seasonDisplayedEnd": day,
                "weeklyStoreEnd": day,
                "stwEventStoreEnd": day,
                "stwWeeklyStoreEnd": day,
                "dailyStoreEnd": day
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
pub async fn keychain(app: web::Data<State>) -> impl Responder {
    HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .body(app.keychain.clone())
}

#[get("/api/version")]
pub async fn fortnite_version() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "app": "fortnite",
        "serverDate": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        "overridePropertiesVersion": "unknown",
        "cln": "2870186",
        "build": "1",
        "moduleName": "Fortnite-Core",
        "buildDate": "2016-02-17T10:16:51.000Z",
        "version": "4.12.0-2870186+++Fortnite+Release-Live",
        "branch": "++Fortnite+Release-Live",
        "modules": {}
    }))
}
