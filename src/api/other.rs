use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/lightswitch/api/service/bulk/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!([
        {
            "serviceInstanceId": "fortnite",
            "status": "UP",
            "message": "Project Era is UP",
            "maintenanceUri": null,
            "overrideCatalogIds": [
                "a7f138b2e51945ffbfdacc1af0541053"
            ],
            "allowedActions": [
                "PLAY",
                "DOWNLOAD"
            ],
            "banned": false,
            "launcherInfoDTO": {
                "appName": "Fortnite",
                "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
                "namespace": "fn"
            }
        }
    ]))
}

#[get("/waitingroom/api/waitingroom")]
pub async fn waitingroom() -> impl Responder {
    HttpResponse::NoContent()
}