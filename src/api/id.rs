use crate::structs::{app::*, discord::*, ALPHABET};
use actix_web::{cookie::Cookie, get, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{doc, DateTime};
use urlencoding::encode;

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
    let user = match get_user(&mut client, auth.access_token.clone(), "@me").await {
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
                    discord_last_token: auth.access_token,
                    discord_refresh_token: auth.refresh_token,
                    discord_id: user.id.clone(),
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
    } else {
        // app.database.users.update_one(
        //     doc! { "discord_id": user.id.clone() },
        //     doc! {
        //         "discord_avatar": user.avatar,
        //         "discord_last_token": auth.access_token,
        //         "discord_refresh_token": auth.refresh_token
        //     },
        //     None
        // ).await.unwrap();
    }

    // unwrap should be alright here since the account should be made above ^
    let user = app
        .database
        .users
        .find_one(doc! { "discord_id": user.id }, None)
        .await
        .unwrap()
        .unwrap();
    let token = nanoid::nanoid!(32, &ALPHABET);

    {
        let mut tokens = app.tokens.write().unwrap();

        tokens.insert(token.clone(), Token::new(Some(user.id), TokenType::Web));
    }

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "/#account"))
        .cookie(Cookie::build("ETOKEN", token).path("/").finish())
        .cookie(
            Cookie::build("EUSERNAME", user.display_name)
                .path("/")
                .finish(),
        )
        .cookie(
            Cookie::build("EDISCORD_AVATAR", user.discord_avatar)
                .path("/")
                .finish(),
        )
        .finish()
}

#[get("/api/discord/url")]
pub async fn discord_url(app: web::Data<State>) -> impl Responder {
    HttpResponse::Ok().body(
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
            app.discord.client_id.clone(),
            encode(&app.discord.oauth_url.clone())
        )
    )
}


#[get("/api/user/@me")]
pub async fn user_info(app: web::Data<State>, req: HttpRequest) -> impl Responder {
    let auth = match req.cookie("ETOKEN") {
        Some(cookie) => cookie,
        None => return HttpResponse::Unauthorized().into(),
    };
    let auth = auth.value().to_string();

    {
        let tokens = app.tokens.read().unwrap();

        if let Some(token) = tokens.get(&auth) {
            let user = match app
                .database
                .users
                .find_one(doc! { "id": token.id.as_ref() }, None)
                .await
                .unwrap()
            {
                Some(user) => user,
                None => return HttpResponse::NotFound().into(),
            };

            return HttpResponse::Ok().json(EUser {
                id: user.id,
                admin: user.admin,
                discord_avatar: user.discord_avatar,
                discord_id: user.discord_id,
                display_name: user.display_name,
            });
        }
    }

    HttpResponse::NotFound().into()
}
