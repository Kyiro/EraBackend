#![allow(non_snake_case)]
#[allow(unused_imports)]
use crate::structs::app::{CItem, CVariant, User};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// most also reused from Ruten for my own insanity
// Fortnite Profile

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub profileRevision: i32,
    pub profileId: String,
    pub profileChangesBaseRevision: i32,
    pub profileChanges: Vec<ProfileChanges>,
    pub profileCommandRevision: i32,
    pub serverTime: String,
    pub responseVersion: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileChanges {
    Full(FullProfile),
    Changed(AttrChanged),
    Stat(StatModified),
}

#[derive(Serialize, Deserialize)]
pub struct StatModified {
    pub changeType: String,
    pub name: String,
    pub value: StatValue,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatValue {
    Vec(Vec<String>),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attributes {
    Bool(bool),
    String(String),
    Variants(Vec<Variant>),
}

#[derive(Serialize, Deserialize)]
pub struct AttrChanged {
    pub changeType: String,
    pub itemId: String,
    pub attributeName: String,
    pub attributeValue: Attributes,
}

#[derive(Serialize, Deserialize)]
pub struct FullProfile {
    pub changeType: String,
    pub profile: FullProfileUpdate,
}

impl FullProfile {
    pub fn new(id: &str) -> Self {
        let id = String::from(id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        Self {
            changeType: String::from("fullProfileUpdate"),
            profile: FullProfileUpdate {
                _id: id.clone(),
                created: now.clone(),
                updated: now.clone(),
                rvn: 1,
                wipeNumber: 1,
                accountId: id,
                profileId: String::from("athena"),
                version: String::from("EraBackend by Kyiro"),
                items: HashMap::new(),
                stats: Stats {
                    attributes: StatsAttributes::None,
                },
                commandRevision: 1,
            },
        }
    }

    pub fn new_athena(cosmetics: &Vec<CItem>, id: &str, profile: User) -> Self {
        let mut full_profile = Self::new(id);

        full_profile.profile.stats.attributes = StatsAttributes::Athena(AthenaAttributes {
            past_seasons: Vec::new(),
            season_match_boost: 0,
            mfa_reward_claimed: true,
            rested_xp_overflow: 0,
            quest_manager: json!({
                "dailyLoginInterval": "2021-06-24T11:24:14.414Z",
                "dailyQuestRerolls": 1
            }),
            book_level: 100,
            season_num: 2,
            book_xp: 999999,
            permissions: Vec::new(),
            season: json!({
                "numWins": 0,
                "numHighBracket": 0,
                "numLowBracket": 0
            }),
            battlestars: 9999,
            vote_data: json!({}),
            book_purchased: true,
            lifetime_wins: 999,
            party_assist_quest: String::new(),
            purchased_battle_pass_tier_offers: json!({}),
            rested_xp_exchange: 1,
            level: 100,
            xp_overflow: 0,
            rested_xp: 0,
            rested_xp_mult: 4.55,
            accountLevel: 9999,
            competitive_identity: json!({}),
            inventory_limit_bonus: 0,
            daily_rewards: json!({}),
            xp: 9999999,
            season_friend_match_boost: 0,
            // cosmetics
            favorite_character: profile.character,
            favorite_backpack: profile.backpack,
            favorite_pickaxe: profile.pickaxe,
            favorite_glider: profile.glider,
            favorite_skydivecontrail: profile.contrail,
            favorite_musicpack: profile.music_pack,
            favorite_loadingscreen: profile.loading,
            favorite_dance: profile.dance,
            favorite_itemwraps: profile.item_wrap,
            // unused cosmetics
            favorite_callingcard: String::new(),
            favorite_consumableemote: String::new(),
            favorite_spray: Vec::new(),
            favorite_hat: String::new(),
            favorite_battlebus: String::new(),
            favorite_mapmarker: String::new(),
            favorite_vehicledeco: String::new(),
            favorite_victorypose: String::new(),
        });

        for i in cosmetics.into_iter() {
            let template = [i.item_type.clone(), i.id.clone()].join(":");

            full_profile.profile.items.insert(
                template.clone(),
                Item::Cosmetic(CosmeticItem {
                    templateId: template,
                    attributes: CosmeticAttributes {
                        max_level_bonus: 0,
                        level: 1,
                        item_seen: true,
                        xp: 0,
                        // TO-DO: Add unlockable edit styles
                        variants: Vec::new(),
                        favorite: false,
                    },
                    quantity: 1,
                }),
            );
        }

        full_profile
    }

    pub fn new_common_core(id: &str) -> Self {
        let mut full_profile = Self::new(id);

        full_profile.profile.stats.attributes = StatsAttributes::CommonCore(CommonCoreAttributes {
            survey_data: json!({}),
            personal_offers: json!({}),
            intro_game_played: false,
            import_friends_claimed: json!({}),
            mtx_purchase_history: json!({}),
            undo_cooldowns: Vec::new(),
            mtx_affiliate_set_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            inventory_limit_bonus: 0,
            current_mtx_platform: String::from("EpicPC"),
            mtx_affiliate: String::from(""),
            weekly_purchases: json!({}),
            daily_purchases: json!({}),
            ban_history: json!({}),
            in_app_purchases: json!({}),
            permissions: Vec::new(),
            undo_timeout: String::from("min"),
            monthly_purchases: json!({}),
            allowed_to_send_gifts: true,
            mfa_enabled: false,
            allowed_to_receive_gifts: true,
            gift_history: json!({}),
        });

        full_profile
    }

    pub fn new_common_public(id: &str) -> Self {
        let mut full_profile = Self::new(id);

        full_profile.profile.stats.attributes =
            StatsAttributes::CommonPublic(CommonPublicAttributes {
                banner_color: String::from(""),
                banner_icon: String::from(""),
                homebase_name: String::from("Project Era"),
            });

        full_profile
    }
}

// pub fn variants(cvariants: &Vec<CVariant>) -> Vec<Variant> {
//     let mut variants: Vec<Variant> = Vec::new();
//     for v in cvariants.iter() {
//         if &v.channel == "JerseyColor" { continue; }
//         // idk if clone is good here but whatever
//         variants.push(Variant {
//             channel: v.channel.clone(),
//             active: v.options.get(0).unwrap().clone(),
//             owned: v.options.clone()
//         });
//     }
//     variants
// }

#[derive(Serialize, Deserialize)]
pub struct FullProfileUpdate {
    pub _id: String,
    pub created: String,
    pub updated: String,
    pub rvn: i32,
    pub wipeNumber: i32,
    pub accountId: String,
    pub profileId: String,
    pub version: String,
    pub items: HashMap<String, Item>,
    pub stats: Stats,
    pub commandRevision: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub attributes: StatsAttributes,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsAttributes {
    Athena(AthenaAttributes),
    Campaign(CampaignAttributes),
    CommonCore(CommonCoreAttributes),
    CommonPublic(CommonPublicAttributes),
    None,
}

#[derive(Serialize, Deserialize)]
pub struct AthenaAttributes {
    pub past_seasons: Vec<Value>,
    pub season_match_boost: i32,
    pub mfa_reward_claimed: bool,
    pub rested_xp_overflow: i32,
    pub quest_manager: Value,
    pub book_level: i32,
    pub season_num: i32,
    pub book_xp: i32,
    pub permissions: Vec<Value>,
    pub season: Value,
    pub battlestars: i32,
    pub vote_data: Value,
    pub book_purchased: bool,
    pub lifetime_wins: i32,
    pub party_assist_quest: String,
    pub purchased_battle_pass_tier_offers: Value,
    pub rested_xp_exchange: i32,
    pub level: i32,
    pub xp_overflow: i32,
    pub rested_xp: i32,
    pub rested_xp_mult: f32,
    pub accountLevel: i32,
    pub competitive_identity: Value,
    pub inventory_limit_bonus: i32,
    pub daily_rewards: Value,
    pub xp: i32,
    pub season_friend_match_boost: i32,
    // cosmetics
    pub favorite_character: String,
    pub favorite_backpack: String,
    pub favorite_pickaxe: String,
    pub favorite_glider: String,
    pub favorite_skydivecontrail: String,
    pub favorite_musicpack: String,
    pub favorite_loadingscreen: String,
    pub favorite_dance: [String; 6],
    pub favorite_itemwraps: [String; 7],
    // unused cosmetics
    pub favorite_callingcard: String,
    pub favorite_consumableemote: String,
    pub favorite_spray: Vec<String>,
    pub favorite_hat: String,
    pub favorite_battlebus: String,
    pub favorite_mapmarker: String,
    pub favorite_vehicledeco: String,
    pub favorite_victorypose: String,
}

#[derive(Serialize, Deserialize)]
pub struct CampaignAttributes {
    pub node_costs: Value,
    pub mission_alert_redemption_record: Value,
    pub rewards_claimed_post_max_level: i32,
    pub collection_book: Value,
    pub mfa_reward_claimed: bool,
    pub quest_manager: Value,
    pub legacy_research_points_spent: i32,
    pub gameplay_stats: Vec<Value>,
    pub permissions: Vec<Value>,
    pub unslot_mtx_spend: i32,
    pub twitch: Value,
    pub client_settings: Value,
    pub research_levels: Value,
    pub level: i32,
    pub xp_overflow: i32,
    pub latent_xp_marker: i32,
    pub inventory_limit_bonus: i32,
    pub xp_lost: i32,
    pub mode_loadouts: Vec<Value>,
    pub daily_rewards: Value,
    pub xp: i32,
    pub packs_granted: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CommonCoreAttributes {
    pub survey_data: Value,
    pub personal_offers: Value,
    pub intro_game_played: bool,
    pub import_friends_claimed: Value,
    pub mtx_purchase_history: Value,
    pub undo_cooldowns: Vec<Value>,
    pub mtx_affiliate_set_time: String,
    pub inventory_limit_bonus: i32,
    pub current_mtx_platform: String,
    pub mtx_affiliate: String,
    pub weekly_purchases: Value,
    pub daily_purchases: Value,
    pub ban_history: Value,
    pub in_app_purchases: Value,
    pub permissions: Vec<Value>,
    pub undo_timeout: String,
    pub monthly_purchases: Value,
    pub allowed_to_send_gifts: bool,
    pub mfa_enabled: bool,
    pub allowed_to_receive_gifts: bool,
    pub gift_history: Value,
}

#[derive(Serialize, Deserialize)]
pub struct CommonPublicAttributes {
    pub banner_color: String,
    pub homebase_name: String,
    pub banner_icon: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Cosmetic(CosmeticItem),
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticItem {
    pub templateId: String,
    pub attributes: CosmeticAttributes,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticAttributes {
    pub max_level_bonus: i32,
    pub level: i32,
    pub item_seen: bool,
    pub xp: i32,
    pub variants: Vec<Variant>,
    pub favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>,
}
