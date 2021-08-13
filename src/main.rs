use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env::{set_var, var};

pub mod api;
pub mod files;
pub mod structs;
pub mod utils;

pub const VERSION: &'static str = "1.2";

// funny macro lol
#[macro_export]
macro_rules! public {
    // URL and File
    [$(($x:expr, $y:expr)),*] => {
        web::scope("/")
        $(
            .route($x, web::to(|| HttpResponse::Ok().body(include_bytes!($y).to_vec())))
        )*
    };
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header(("Location", "/index.html"))
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

    let state = web::Data::new(structs::app::State::new().await);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // .service(index)
            .service(version)
            .service(
                web::scope("/account")
                    .service(api::account::oauth_token)
                    .service(api::account::oauth_verify)
                    .service(api::account::external_auths)
                    .service(api::account::kill_sessions)
                    .service(api::account::kill_sessions_id)
                    .service(api::account::personal_account)
                    .service(api::account::personal_account_query),
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
                    // .service(api::profile::equip_battle_royale)
                    .service(api::profile::query_profile)
                    .service(api::profile::other),
            )
            .service(
                web::scope("/id")
                    .service(api::id::discord_oauth)
                    .service(api::id::discord_url)
                    .service(api::id::user_info)
            )
            .service(web::scope("/lightswitch").service(api::lightswitch::status))
            .service(api::other::party_user)
            .service(api::other::waitingroom)
            .service(public![
                ("", "../resources/public/index.html"),
                ("index.html", "../resources/public/index.html"),
                ("style.css", "../resources/public/style.css"),
                ("script.js", "../resources/public/script.js"),
                ("favicon.ico", "../resources/public/favicon.ico"),
                ("svg/book.svg", "../resources/public/svg/book.svg"),
                ("svg/discord.svg", "../resources/public/svg/discord.svg"),
                ("svg/download.svg", "../resources/public/svg/download.svg"),
                ("svg/home.svg", "../resources/public/svg/home.svg"),
                ("svg/people.svg", "../resources/public/svg/people.svg"),
                ("svg/user.svg", "../resources/public/svg/user.svg"),
                ("img/danii.webp", "../resources/public/img/danii.webp"),
                ("img/kemo.webp", "../resources/public/img/kemo.webp"),
                ("img/kyiro.webp", "../resources/public/img/kyiro.webp"),
                ("img/mix.webp", "../resources/public/img/mix.webp"),
                ("img/ozne.webp", "../resources/public/img/ozne.webp"),
                ("img/robot.webp", "../resources/public/img/robot.webp"),
                ("img/sizzy.webp", "../resources/public/img/sizzy.webp")
            ])
    })
    .bind(format!(
        "0.0.0.0:{}",
        var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}
