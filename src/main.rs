use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env::{set_var, var};

pub mod api;
pub mod files;
pub mod structs;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header(("Location", "https://erafn.glitch.me/"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    if let Err(_) = var("RUST_LOG") {
        set_var("RUST_LOG", "info");
    }
    // pwetty
    pretty_env_logger::init();

    let state = web::Data::new({
        let mut state = structs::app::State::new();

        // get cosmetics from cosmetics.json and use a fallback when it fails
        state.cosmetics = files::cosmetics().unwrap_or({
            let data = include_str!("../resources/cosmetics.json");
            serde_json::from_str(data)?
        });
        state.game =
            files::game().unwrap_or(include_str!("../resources/fortnite-game.json").to_string());

        state
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(
                web::scope("/account")
                    .service(api::account::oauth_token)
                    .service(api::account::oauth_verify)
                    .service(api::account::external_auths)
                    .service(api::account::kill_sessions)
                    .service(api::account::kill_sessions_id)
                    .service(api::account::personal_account),
            )
            .service(web::scope("/content").service(api::content::fortnite_game))
            .service(
                web::scope("/fortnite")
                    .service(api::cloudstorage::system)
                    .service(api::cloudstorage::system_config)
                    .service(api::cloudstorage::system_file)
                    .service(api::cloudstorage::user)
                    .service(api::cloudstorage::user_file)
                    .service(api::cloudstorage::put_user_file)
                    .service(api::fortnite::catalog)
                    .service(api::fortnite::enabled_features)
                    .service(api::fortnite::find_player)
                    .service(api::fortnite::keychain)
                    .service(api::fortnite::play_on_platform)
                    .service(api::fortnite::receipts)
                    .service(api::fortnite::timeline)
                    .service(api::fortnite::version_check)
                    .service(api::fortnite::world_info)
                    .service(api::profile::client_quest_login)
                    .service(api::profile::equip_battle_royale)
                    .service(api::profile::query_profile)
                    .service(api::profile::other),
            )
            .service(api::other::status)
            .service(api::other::waitingroom)
    })
    .bind(format!(
        "0.0.0.0:{}",
        var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}
