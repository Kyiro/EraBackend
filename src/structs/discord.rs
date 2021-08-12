use crate::structs::app::DiscordApp;
use serde::{Deserialize, Serialize};

const URL: &'static str = "https://discord.com/api";

#[derive(Deserialize)]
pub struct OAuthQuery {
    pub code: String,
}

#[derive(Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: i32,
    pub flags: i32,
    pub banner: Option<String>,
    pub banner_color: Option<String>,
    pub accent_color: Option<String>,
    pub locale: String,
    pub mfa_enabled: bool,
}

#[derive(Serialize)]
pub struct OAuthForm {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

// TO-DO: Switch to Result
pub async fn auth_code(
    client: &mut reqwest::Client,
    app: &DiscordApp,
    code: String,
) -> Option<OAuthToken> {
    let req = client
        .post(URL.to_owned() + "/oauth2/token")
        .form(&[
            ("client_id", app.client_id.clone()),
            ("client_secret", app.secret.clone()),
            ("grant_type", String::from("authorization_code")),
            ("code", code),
            ("redirect_uri", app.oauth_url.clone()),
        ])
        .send()
        .await
        .ok()?;

    req.json::<OAuthToken>().await.ok()
}

pub async fn get_user(
    client: &mut reqwest::Client,
    auth: String,
    user: &str,
) -> Option<DiscordUser> {
    let req = client
        .get(URL.to_owned() + "/users/" + &user)
        .header("Authorization", "Bearer ".to_owned() + &auth.to_owned())
        .send()
        .await
        .ok()
        .unwrap();

    req.json().await.ok()
}
