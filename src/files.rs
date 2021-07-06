use crate::structs::app::CItem;
use std::fs::read_to_string;

pub fn cosmetics() -> std::io::Result<Vec<CItem>> {
    Ok(serde_json::from_str(&read_to_string("cosmetics.json")?)?)
}

pub fn game() -> std::io::Result<String> {
    Ok(read_to_string("fortnite-game.json")?)
}