use crate::files;
use mongodb::bson::DateTime;
use mongodb::{error::Result as MDResult, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sha2::Sha256;
use std::collections::HashMap;
use std::env::var;
use std::sync::{Arc, RwLock};

pub struct State {
    pub cosmetics: Vec<CItem>,
    pub game: String,
    pub keychain: String,
    pub database: Database,
    pub discord: DiscordApp,
    // Yeah! Um...
    pub tokens: Arc<RwLock<HashMap<String, Token>>>,
}

impl State {
    pub async fn new() -> Self {
        Self {
            cosmetics: files::cosmetics(),
            game: files::game(),
            keychain: files::keychain(),
            database: Database::new().await.expect("Couldn't connect to DB"),
            discord: DiscordApp::new(),
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub struct DiscordApp {
    pub client_id: String,
    pub secret: String,
    pub oauth_url: String,
}

impl DiscordApp {
    pub fn new() -> Self {
        Self {
            client_id: var("DISCORD_CLIENT").expect("DISCORD_CLIENT Not Present"),
            secret: var("DISCORD_SECRET").expect("DISCORD_SECRET Not Present"),
            oauth_url: var("DISCORD_URL").expect("DISCORD_URL Not Present"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub id: String,
    pub variants: Vec<CVariant>,
}

impl CItem {
    pub fn new() -> Self {
        Self {
            item_type: String::new(),
            id: String::new(),
            variants: Vec::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CVariant {
    pub channel: String,
    pub options: Vec<String>,
}

pub struct Database {
    pub athena: Collection<Athena>,
    pub cloudstorage: Collection<CloudStorage>,
    pub tokens: Collection<RefreshToken>,
    pub users: Collection<User>,
}

impl Database {
    pub async fn new() -> MDResult<Self> {
        let client_options = {
            let mut options =
                ClientOptions::parse(var("MONGODB").expect("MONGODB Not Present")).await?;
            options.app_name = Some("Project Era".to_string());
            options
        };

        let client = Client::with_options(client_options)?;
        let db = client.database(&var("DB_NAME").expect("DB_NAME Not Present"));

        Ok(Self {
            athena: db.collection::<Athena>("athena"),
            cloudstorage: db.collection::<CloudStorage>("cloudstorage"),
            tokens: db.collection::<RefreshToken>("tokens"),
            users: db.collection::<User>("users"),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Athena {
    pub id: String,
    pub favourites: Vec<String>,
    pub battlebus: String,
    pub character: String,
    pub backpack: String,
    pub pickaxe: String,
    pub glider: String,
    pub contrail: String,
    pub music_pack: String,
    pub loading: String,
    pub dance: [String; 6],
    pub item_wrap: [String; 7],
}

impl Athena {
    pub fn new(id: String) -> Self {
        Self {
            id,
            favourites: Vec::new(),
            battlebus: String::new(),
            character: String::from("AthenaCharacter:cid_005_athena_commando_m_default"),
            backpack: String::new(),
            pickaxe: String::from("AthenaPickaxe:defaultpickaxe"),
            glider: String::from("AthenaGlider:defaultglider"),
            contrail: String::from(""),
            music_pack: String::new(),
            loading: String::new(),
            dance: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            item_wrap: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorage {
    pub id: String,
    pub files: HashMap<String, CloudStorageFile>,
}

impl CloudStorage {
    pub fn new(id: String) -> Self {
        Self {
            id,
            files: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorageFile {
    pub data: Vec<u8>,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    pub last_updated: DateTime,
}

impl CloudStorageFile {
    pub fn new(data: Vec<u8>) -> Self {
        // i hate how these hashes work in rust
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&data);
        sha256.update(&data);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();
        let length = data.len();

        Self {
            data,
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: length,
            last_updated: DateTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    // i have cool plans for this
    pub admin: bool,
    pub creation_time: DateTime,
    pub discord_avatar: String,
    pub discord_last_token: String,
    pub discord_refresh_token: String,
    pub discord_id: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: String,
    pub date: DateTime,
    pub token: String,
}

pub struct Token {
    pub token_type: TokenType,
    pub id: Option<String>,
}

impl Token {
    pub fn new(id: Option<String>, token_type: TokenType) -> Self {
        Self { id, token_type }
    }
}

pub enum TokenType {
    ClientCredentials,
    Bearer,
    Web,
}

#[derive(Serialize)]
pub struct EUser {
    pub id: String,
    pub admin: bool,
    pub discord_avatar: String,
    pub discord_id: String,
    pub display_name: String,
}
