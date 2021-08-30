use crate::structs::app::{CItem, Shops};
use serde_json::{json, Value};
use std::fs::read_to_string;

pub fn cosmetics() -> Vec<CItem> {
    serde_json::from_str(
        &read_to_string("cosmetics.json")
            .unwrap_or(include_str!("../resources/cosmetics.json").to_string()),
    )
    .unwrap_or(Vec::new())
}

pub fn game() -> String {
    read_to_string("fortnite-game.json")
        .unwrap_or(include_str!("../resources/fortnite-game.json").to_string())
}

pub fn keychain() -> String {
    read_to_string("keychain.json")
        .unwrap_or(include_str!("../resources/keychain.json").to_string())
}

pub fn shops() -> Shops {
    serde_json::from_str(
        &read_to_string("shops.json")
            .unwrap_or(include_str!("../resources/shops.json").to_string()),
    )
    .unwrap_or(Shops::new())
}

pub fn events() -> Value {
    serde_json::from_str(
        &read_to_string("events.json")
            .unwrap_or(include_str!("../resources/events.json").to_string()),
    )
    .unwrap_or(json!({}))
}