use std::{env, io};

use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;
use handlebars::{DirectorySourceOptions, Handlebars};

mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut handlebars = Handlebars::new();
    let _ = handlebars.register_templates_directory(
        "./templates",
        DirectorySourceOptions {
            hidden: false,
            temporary: false,
            tpl_extension: ".hbs".to_owned(),
        },
    );

    // Checking if `STEAM_API_KEY` is there on startup
    let _ = env::var("STEAM_API_KEY").expect("missing `STEAM_API_KEY` environment variable");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(handlebars.clone()))
            .service(routes::main::main)
    })
    .bind((
        "127.0.0.1",
        env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .unwrap(),
    ))?
    .run()
    .await
}
