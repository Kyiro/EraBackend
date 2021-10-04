use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env::{set_var, var};

pub mod api;
pub mod files;
pub mod structs;
pub mod utils;

pub const VERSION: &'static str = "1.2";
pub const CLOUDSTORAGE: [(&'static str, &'static str); 3] = [
    ("DefaultGame.ini", include_str!("../resources/cloudstorage/DefaultGame.ini")),
    ("DefaultRuntimeOptions.ini", include_str!("../resources/cloudstorage/DefaultRuntimeOptions.ini")),
    ("DefaultEngine.ini", include_str!("../resources/cloudstorage/DefaultEngine.ini"))
];

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "https://erafn.glitch.me/"))
        .finish()
}

#[post("/VersionRequest")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body(VERSION)
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

        state.cosmetics = files::cosmetics();
        state.events = files::events();
        state.game = files::game();
        state.keychain = files::keychain();
        state.shops = files::shops();

        state
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(version)
            .service(
                web::scope("/account")
                    .service(api::account::accounts_metadata)
                    .service(api::account::oauth_token)
                    .service(api::account::oauth_verify)
                    .service(api::account::external_auths)
                    .service(api::account::kill_sessions)
                    .service(api::account::kill_sessions_id)
                    .service(api::account::personal_account)
                    .service(api::account::personal_account_query)
                    .service(api::account::ssodomains),
            )
            .service(
			    web::scope("/content")
				    .service(api::content::fortnite_game)
					.service(api::content::fortnite_game_)
			    )
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
                    .service(api::fortnite::fortnite_version)
                    .service(api::fortnite::keychain)
                    .service(api::fortnite::play_on_platform)
                    .service(api::fortnite::receipts)
                    .service(api::fortnite::timeline)
                    .service(api::fortnite::twitch)
                    .service(api::fortnite::version_check)
                    .service(api::fortnite::version_check_v2)
                    .service(api::fortnite::world_info)
                    .service(api::profile::client_quest_login)
                    .service(api::profile::equip_battle_royale)
                    .service(api::profile::query_profile)
                    .service(api::profile::other),
            )
            .service(
                web::scope("/lightswitch")
                    .service(api::lightswitch::bulk_status)
                    .service(api::lightswitch::fortnite_status),
            )
            .service(api::other::eulatracking)
            .service(api::other::events)
            .service(api::other::events_history)
            .service(api::other::events_history_window)
            .service(api::other::datarouter)
            .service(api::other::friends)
            .service(api::other::blocklist)
            .service(api::other::recent_players)
            .service(api::other::party_user)
            .service(api::other::settings)
            .service(api::other::waitingroom)
    })
    .bind(format!(
        "0.0.0.0:{}",
        var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}
