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
        let player = players.first();

        if player.is_none() {
            let body = hb
                .render(
                    "main",
                    &json!({
                        "title": "Unprocessable Entity",
                        "description": format!("Invalid user ID ({}) passed", steam_id.0)
                    }),
                )
                .unwrap();
            return HttpResponse::UnprocessableEntity().body(body);
        }

        if player.unwrap().game_id.is_none() {
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
            let description = format!(
                "Currently playing {}",
                player.unwrap().game_extra_info.clone().unwrap()
            );
            let body = hb
                    .render(
                        "main",
                        &json!({
                                "icon": format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/logo.png", player.unwrap().game_id.clone().unwrap()),
                                "title": player.unwrap().game_extra_info,
                                "description": description
                            }),
                    )
                    .unwrap();

            return HttpResponse::Ok().body(body);
        }
    }

    HttpResponse::Ok().finish()
}
