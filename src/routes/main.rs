use std::env;

use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use handlebars::Handlebars;
use serde_json::json;
use steam_rs::{steam_id::SteamId, Steam};

#[get("/{id}")]
pub async fn main(path: Path<u64>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let steam = Steam::new(&env::var("STEAM_API_KEY").unwrap());
    let steam_id = SteamId::new(path.into_inner());

    if let Ok(players) = steam.get_player_summaries(vec![steam_id]).await {
        for p in players.iter() {
            if p.game_id.is_none() {
                let body = hb
                    .render(
                        "main",
                        &json!({
                            "title": "Offline",
                            "description": "Currently not playing anything"
                        }),
                    )
                    .unwrap();

                return HttpResponse::Ok().body(body);
            } else {
                let description =
                    format!("Currently playing {}", p.game_extra_info.clone().unwrap());
                let body = hb
                    .render(
                        "main",
                        &json!({
                                "icon": format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/logo.png", p.game_id.clone().unwrap()),
                                "title": p.game_extra_info,
                                "description": description
                            }),
                    )
                    .unwrap();

                return HttpResponse::Ok().body(body);
            }
        }
    }

    HttpResponse::Ok().finish()
}
