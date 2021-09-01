use actix_web::{get, post, web, http, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

mod get_game_cards;

fn env_get(x: String) -> String {
    for (key, value) in env::vars() {
        if key == x {
            return value;
        }
    }
    return "".to_owned();
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut address: String = "127.0.0.1:".to_owned();
    let port: String = env_get("PORT".to_owned());
    address.push_str(&port);
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::new()
        .allowed_origin("http://localhost:3000")
        .finish()
    )
        .route("/api/get_game_cards", web::get().to(get_game_cards::get_cards))
    })
    .bind(address)?
    .run()
    .await
}
