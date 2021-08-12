use crate::structs::{app::*, discord::*, ALPHABET};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::{doc, DateTime};

// TO-DO: Get rid of the unwraps
#[get("/api/discord/oauth2")]
pub async fn discord_oauth(app: web::Data<State>, query: web::Query<OAuthQuery>) -> impl Responder {
    if query.code.len() != 30 {
        return HttpResponse::BadRequest().into();
    }

    // TO-DO: Move to storing the client somewhere
    let mut client = reqwest::Client::new();

    let auth = match auth_code(&mut client, &app.discord, query.code.clone()).await {
        Some(d) => d,
        None => return HttpResponse::InternalServerError().into(),
    };
    let user = match get_user(&mut client, auth.access_token, "@me").await {
        Some(d) => d,
        None => return HttpResponse::InternalServerError().into(),
    };

    if let None = app
        .database
        .users
        .find_one(doc! { "discord_id": user.id.clone() }, None)
        .await
        .unwrap()
    {
        let account_id = nanoid::nanoid!(32, &ALPHABET);
        app.database
            .users
            .insert_one(
                User {
                    id: account_id.clone(),
                    admin: false,
                    creation_time: DateTime::now(),
                    discord_avatar: user.avatar,
                    discord_refresh_token: auth.refresh_token,
                    discord_id: user.id,
                    display_name: user.username,
                },
                None,
            )
            .await
            .unwrap();

        app.database
            .athena
            .insert_one(Athena::new(account_id.clone()), None)
            .await
            .unwrap();
        app.database
            .cloudstorage
            .insert_one(CloudStorage::new(account_id.clone()), None)
            .await
            .unwrap();
    }

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "/"))
        .finish()
}
