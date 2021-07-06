use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct State {
    pub cosmetics: Vec<CItem>,
    pub game: String,
    // Yeah! Um...
    pub users: Arc<RwLock<HashMap<String, User>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            cosmetics: Vec::new(),
            game: String::new(),
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
}

// TO-DO: Add Variants
// I think they **should** be defined in every skin afaik
// And it's set in EquipBattleRoyaleCustomization
// Just like newer SetCosmeticLockerSlot it's in variantUpdates
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
            character: String::new(),
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
