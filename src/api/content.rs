use crate::structs::app::State;
use crate::utils::get_build;
use actix_web::*;
use serde_json::json;

// yes it's different lol
#[get("/api/pages/fortnite-game/")]
pub async fn fortnite_game_(
    app: web::Data<State>,
    req: HttpRequest
) -> impl Responder {
    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let build = get_build(useragent).unwrap();
    let mut game = app.game.clone();
    
    game["dynamicbackgrounds"] = json!({
        "jcr:isCheckedOut": true,
        "backgrounds": {
            "backgrounds": [
                {
                    "backgroundimage": "https://cdn2.unrealengine.com/fortnitemares-wrath-of-the-cube-queen-2048x1024-5a3d045848be.png",
                    "stage": format!("season{}", build.season),
                    "_type": "DynamicBackground",
                    "key": "lobby"
                }
            ],
            "_type": "DynamicBackgroundList"
        },
        "_title": "dynamicbackgrounds",
        "_noIndex": false,
        "jcr:baseVersion": "a7ca237317f1e734b3b9d6-8d7b-42cd-bcf1-19decf60552a",
        "_activeDate": "2018-08-06T19:00:26.217Z",
        "lastModified": "2018-08-06T19:00:26.217Z",
        "_locale": "en-US"
    });
    
    HttpResponse::Ok()
        .json(game)
}

#[get("/api/pages/fortnite-game")]
pub async fn fortnite_game(
    app: web::Data<State>,
    req: HttpRequest
) -> impl Responder {
    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let build = get_build(useragent).unwrap();
    let mut game = app.game.clone();
    
    game["dynamicbackgrounds"] = json!({
        "jcr:isCheckedOut": true,
        "backgrounds": {
            "backgrounds": [
                {
                    "backgroundimage": "https://cdn2.unrealengine.com/fortnitemares-wrath-of-the-cube-queen-2048x1024-5a3d045848be.png",
                    "stage": format!("season{}", build.season),
                    "_type": "DynamicBackground",
                    "key": "lobby"
                }
            ],
            "_type": "DynamicBackgroundList"
        },
        "_title": "dynamicbackgrounds",
        "_noIndex": false,
        "jcr:baseVersion": "a7ca237317f1e734b3b9d6-8d7b-42cd-bcf1-19decf60552a",
        "_activeDate": "2018-08-06T19:00:26.217Z",
        "lastModified": "2018-08-06T19:00:26.217Z",
        "_locale": "en-US"
    });
    
    HttpResponse::Ok()
        .json(game)
}
