use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Shops = HashMap<usize, HashMap<String, Vec<ShopItem>>>;

#[derive(Clone)]
pub struct State {
    pub cosmetics: Vec<CItem>,
    pub events: Value,
    pub game: Value,
    pub keychain: String,
    pub shops: Shops,
    // Yeah! Um...
    pub users: Arc<RwLock<HashMap<String, User>>>,
}

#[derive(Clone, Deserialize)]
pub struct ShopItem {
    pub id: String,
    pub da: Option<String>,
    pub price: usize,
    pub categories: Option<Vec<String>>
}

impl State {
    pub fn new() -> Self {
        Self {
            cosmetics: Vec::new(),
            events: json!({}),
            game: json!({}),
            keychain: String::new(),
            shops: Shops::new(),
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_user(&self, id: &str) -> User {
        let users = self.users.read().unwrap();

        match users.get(id) {
            Some(user) => user.clone(),
            None => {
                drop(users);
                let new_user = User::new();

                {
                    // RwLocks lock when writing so seperate scope
                    let mut users = self.users.write().unwrap();
                    users.insert(id.to_string(), new_user.clone());
                }

                new_user
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone)]
pub struct User {
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

impl User {
    pub fn new() -> Self {
        Self {
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
