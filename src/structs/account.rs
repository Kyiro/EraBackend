use crate::structs::ALPHABET;
use base64::decode;
use chrono::{prelude::*, Duration};
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientCreds {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: String,
    pub token_type: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
}

#[derive(Serialize)]
pub struct BearerToken {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: String,
    pub token_type: String,
    pub refresh_token: String,
    pub refresh_expires: i32,
    pub refresh_expires_at: String,
    pub account_id: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub app: String,
    pub in_app_id: String,
}

fn decode_basic(token: &str) -> String {
    String::from_utf8(decode(token.replace("basic ", "")).unwrap())
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[0]
        .to_string()
}

impl ClientCreds {
    pub fn new(basic: &str) -> Self {
        Self {
            access_token: nanoid::nanoid!(32, &ALPHABET),
            expires_in: 2147483647,
            expires_at: (Utc::now() + Duration::minutes(2147483647))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            token_type: String::from("bearer"),
            client_id: decode_basic(basic),
            internal_client: true,
            client_service: String::from("fortnite"),
        }
    }
}

impl BearerToken {
    pub fn new(basic: &str) -> Self {
        let acc_id = nanoid::nanoid!(8, &ALPHABET);
        Self {
            access_token: nanoid::nanoid!(32, &ALPHABET),
            expires_in: 2147483647,
            expires_at: (Utc::now() + Duration::minutes(2147483647))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            token_type: String::from("bearer"),
            refresh_token: nanoid::nanoid!(32, &ALPHABET),
            refresh_expires: 2147483647,
            refresh_expires_at: (Utc::now() + Duration::minutes(2147483647))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            account_id: acc_id.clone(),
            client_id: decode_basic(basic),
            internal_client: true,
            client_service: String::from("fortnite"),
            display_name: String::from("Project Era"),
            app: String::from("fortnite"),
            in_app_id: acc_id,
        }
    }
}
