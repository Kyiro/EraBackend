use crate::structs::app::{CItem, State};
use crate::structs::profile::*;
use crate::utils::{get_build, Build};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use chrono::prelude::*;
use serde::Deserialize;
use serde_json::json;

// reused from Ruten
fn create(profile_id: String, change: Vec<ProfileChanges>, rvn: Option<i32>) -> Profile {
    Profile {
        profileRevision: rvn.unwrap_or(1) + 1,
        profileId: profile_id,
        profileChangesBaseRevision: rvn.unwrap_or(2),
        profileChanges: change,
        profileCommandRevision: rvn.unwrap_or(1) + 1,
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

#[derive(Deserialize)]
pub struct QueryProfile {}

#[post("/api/game/v2/profile/{id}/client/QueryProfile")]
pub async fn query_profile(
    app: web::Data<State>,
    _: web::Json<QueryProfile>,
    query: web::Query<Query>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let query = query.into_inner();
    let id = id.into_inner();
    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let build = get_build(useragent).unwrap_or(Build::default());

    match query.profile_id.as_str() {
        "athena" => HttpResponse::Ok().json(create(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_athena(
                &app.cosmetics,
                &id,
                app.get_user(&id),
                build.season,
            ))],
            None,
        )),
        "profile0" => HttpResponse::Ok().json(create(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_profile0(&id))],
            None
        )),
        "common_core" => HttpResponse::Ok().json(create(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_common_core(&id))],
            None,
        )),
        "common_public" => HttpResponse::Ok().json(create(
            query.profile_id,
            vec![ProfileChanges::Full(FullProfile::new_common_public(&id))],
            None,
        )),
        _ => HttpResponse::Ok().json(create(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new(&id, &query.profile_id))],
            Some(query.rvn)
        )),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipBattleRoyaleCustomization {
    pub item_to_slot: String,
    pub slot_name: String,
    pub index: Option<usize>,
    pub variants: Option<Vec<Variant>>,
}

#[post("/api/game/v2/profile/{id}/client/EquipBattleRoyaleCustomization")]
pub async fn equip_battle_royale(
    app: web::Data<State>,
    body: web::Json<EquipBattleRoyaleCustomization>,
    query: web::Query<Query>,
    id: web::Path<String>,
) -> impl Responder {
    let body = body.into_inner();
    let query = query.into_inner();
    let id = id.into_inner();
    let cosmetic = {
        let id = body
            .item_to_slot
            .clone()
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap_or(&"")
            .to_string();
        match app.cosmetics.iter().find(|c| c.id == id) {
            Some(data) => data.clone(),
            None => {
                if body.item_to_slot == "" {
                    CItem::new()
                } else {
                    return HttpResponse::BadRequest().into();
                }
            }
        }
    };

    let idx = body.index.unwrap_or(0);

    {
        // make new user if it doesn't exist
        app.get_user(&id);
        let mut profile = app.users.write().unwrap();
        let profile = profile.get_mut(&id).unwrap();

        let slot = match body.slot_name.as_str() {
            "Character" => &mut profile.character,
            "Dance" => &mut profile.dance[idx],
            "ItemWrap" => &mut profile.item_wrap[idx],
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

    let favorite_slot = if body.slot_name == "ItemWrap" {
        String::from("itemwraps")
    } else { body.slot_name.to_lowercase() };
    
    changes.push(ProfileChanges::Stat(StatModified {
        changeType: String::from("statModified"),
        name: "favorite_".to_owned() + &favorite_slot,
        value: match body.slot_name.as_str() {
            "Dance" => StatValue::Vec(app.get_user(&id).dance.to_vec()),
            "ItemWrap" => StatValue::Vec(app.get_user(&id).item_wrap.to_vec()),
            _ => StatValue::String(body.item_to_slot.clone())
        },
    }));

    if let Some(variants) = body.variants {
        if variants.len() != 0 {
            changes.push(ProfileChanges::Changed(AttrChanged {
                changeType: String::from("itemAttrChanged"),
                itemId: body.item_to_slot,
                attributeName: String::from("variants"),
                attributeValue: Attributes::Variants(build_variants(variants, cosmetic.variants)),
            }))
        }
    }

    HttpResponse::Ok().json(create(String::from("athena"), changes, Some(query.rvn)))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCosmeticLockerSlot {
    pub locker_item: String,
    pub category: String,
    pub item_to_slot: String,
    pub slot_index: usize
}

#[post("/api/game/v2/profile/{id}/client/SetCosmeticLockerSlot")]
pub async fn set_cosmetic_locker_slot(
    app: web::Data<State>,
    body: web::Json<SetCosmeticLockerSlot>,
    query: web::Query<Query>,
    id: web::Path<String>,
) -> impl Responder {
    let body = body.into_inner();
    let query = query.into_inner();
    let id = id.into_inner();
    // let cosmetic = {
    //     let id = body
    //         .item_to_slot
    //         .clone()
    //         .split(":")
    //         .collect::<Vec<&str>>()
    //         .get(1)
    //         .unwrap_or(&"")
    //         .to_string();
    //     match app.cosmetics.iter().find(|c| c.id == id) {
    //         Some(data) => data.clone(),
    //         None => {
    //             if body.item_to_slot == "" {
    //                 CItem::new()
    //             } else {
    //                 return HttpResponse::BadRequest().into();
    //             }
    //         }
    //     }
    // };

    {
        // make new user if it doesn't exist
        app.get_user(&id);
        let mut profile = app.users.write().unwrap();
        let profile = profile.get_mut(&id).unwrap();

        let slot = match body.category.as_str() {
            "Character" => &mut profile.character,
            "Dance" => &mut profile.dance[body.slot_index],
            "ItemWrap" => &mut profile.item_wrap[body.slot_index],
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
    
    let profile = app.get_user(&id);
    
    changes.push(ProfileChanges::Changed(AttrChanged {
        changeType: String::from("itemAttrChanged"),
        itemId: body.locker_item,
        attributeName: String::from("locker_slots_data"),
        attributeValue: Attributes::Other(json!({
            "slots": {
                "SkyDiveContrail": {
                    "items": [ profile.contrail ],
                    "activeVariants": []
                },
                "MusicPack": {
                    "items": [ profile.music_pack ],
                    "activeVariants": []
                },
                "Character": {
                    "items": [ profile.character ],
                    "activeVariants": []
                },
                "Backpack": {
                    "items": [ profile.backpack ],
                    "activeVariants": []
                },
                "Glider": {
                    "items": [ profile.glider ],
                    "activeVariants": []
                },
                "Pickaxe": {
                    "items": [ profile.pickaxe ],
                    "activeVariants": []
                },
                "ItemWrap": {
                    "items": profile.item_wrap,
                    "activeVariants": []
                },
                "LoadingScreen": {
                    "items": [ profile.loading ],
                    "activeVariants": []
                },
                "Dance": {
                    "items": profile.dance,
                    "activeVariants": []
                }
            }
        })),
    }));

    HttpResponse::Ok().json(create(String::from("athena"), changes, Some(query.rvn)))
}

#[post("/api/game/v2/profile/{id}/client/ClientQuestLogin")]
pub async fn client_quest_login(query: web::Query<Query>) -> impl Responder {
    let query = query.into_inner();

    HttpResponse::Ok().json(create(query.profile_id, Vec::new(), None))
}

#[post("/api/game/v2/profile/{id}/client/{action}")]
pub async fn other(query: web::Query<Query>) -> impl Responder {
    let query = query.into_inner();
    HttpResponse::Ok().json(create(query.profile_id, Vec::new(), Some(query.rvn)))
}
