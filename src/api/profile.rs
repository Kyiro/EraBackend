use crate::structs::app::State;
use crate::structs::profile::*;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::Deserialize;

// reused from Ruten
fn create(profile_id: String, change: Vec<ProfileChanges>, rvn: Option<i32>) -> Profile {
    Profile {
        profileRevision: rvn.unwrap_or(0) + 1,
        profileId: profile_id,
        profileChangesBaseRevision: rvn.unwrap_or(1),
        profileChanges: change,
        profileCommandRevision: rvn.unwrap_or(0) + 1,
        serverTime: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        responseVersion: 1,
    }
}

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub rvn: i32,
}

#[post("/api/game/v2/profile/{id}/client/QueryProfile")]
pub async fn query_profile(
    app: web::Data<State>,
    query: web::Query<Query>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let query = query.into_inner();
    
    match query.profile_id.as_str() {
        "athena" | "profile0" => {
            let cosmetics = &app.cosmetics;
            let profile = app.get_user(&id);

            HttpResponse::Ok().json(create(
                query.profile_id,
                vec![ProfileChanges::Full(FullProfile::new_athena(
                    cosmetics, &id, profile,
                ))],
                None,
            ))
        }
        // "common_core" => HttpResponse::Ok().json(create(
        //     query.profile_id,
        //     vec![ProfileChanges::Full(FullProfile::new_common_core(&id))],
        //     None,
        // )),
        // "common_public" => HttpResponse::Ok().json(create(
        //     query.profile_id,
        //     vec![ProfileChanges::Full(FullProfile::new_common_public(&id))],
        //     None,
        // )),
        _ => HttpResponse::Ok().json(create(query.profile_id, Vec::new(), None)),
    }
}

#[derive(Deserialize)]
pub struct EquipBattleRoyaleCustomization {
    #[serde(rename = "itemToSlot")]
    pub item_to_slot: String,
    #[serde(rename = "slotName")]
    pub slot_name: String,
    #[serde(rename = "indexWithinSlot")]
    pub index: usize,
    #[serde(rename = "variantUpdates")]
    pub variants: Option<Vec<Variant>>,
}

#[post("/api/game/v2/profile/{id}/client/EquipBattleRoyaleCustomization")]
pub async fn equip_battle_royale(
    app: web::Data<State>,
    body: web::Json<EquipBattleRoyaleCustomization>,
    query: web::Query<Query>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let body = body.into_inner();
    let query = query.into_inner();

    // poor spam protection lol
    if body.item_to_slot.len() > 100 {
        return HttpResponse::BadRequest().into();
    }

    {
        // make new user if it doesn't exist
        app.get_user(&id);
        let mut profile = app.users.write().unwrap();
        let profile = profile.get_mut(&id).unwrap();

        let slot = match body.slot_name.as_str() {
            "Character" => &mut profile.character,
            "Dance" => &mut profile.dance[body.index],
            "ItemWrap" => &mut profile.item_wrap[body.index],
            "Backpack" => &mut profile.backpack,
            "Pickaxe" => &mut profile.pickaxe,
            "Glider" => &mut profile.glider,
            "SkyDiveContrail" => &mut profile.contrail,
            "MusicPack" => &mut profile.music_pack,
            "LoadingScreen" => &mut profile.loading,
            _ => &mut profile.character,
        };

        *slot = body.item_to_slot.clone();
    }

    let mut changes: Vec<ProfileChanges> = Vec::new();

    changes.push(ProfileChanges::Stat(StatModified {
        changeType: String::from("statModified"),
        name: ["favorite", &body.slot_name.to_lowercase()].join("_"),
        value: if &body.slot_name == "Dance" || &body.slot_name == "ItemWrap" {
            StatValue::Vec(app.get_user(&id).dance.to_vec())
        } else {
            StatValue::String(body.item_to_slot.clone())
        },
    }));

    if let Some(variants) = body.variants {
        if variants.len() != 0 {
            changes.push(ProfileChanges::Changed(AttrChanged {
                changeType: String::from("itemAttrChanged"),
                itemId: body.item_to_slot,
                attributeName: String::from("variants"),
                attributeValue: Attributes::Variants(variants),
            }))
        }
    }

    HttpResponse::Ok().json(create(String::from("athena"), changes, Some(query.rvn)))
}

#[post("/api/game/v2/profile/{id}/client/ClientQuestLogin")]
pub async fn client_quest_login(
    query: web::Query<Query>,
    web::Path(_): web::Path<()>,
) -> impl Responder {
    let query = query.into_inner();

    HttpResponse::Ok().json(create(query.profile_id, Vec::new(), None))
}

#[post("/api/game/v2/profile/{id}/client/{action}")]
pub async fn other(query: web::Query<Query>) -> impl Responder {
    let query = query.into_inner();
    HttpResponse::Ok().json(create(query.profile_id, Vec::new(), Some(query.rvn)))
}
