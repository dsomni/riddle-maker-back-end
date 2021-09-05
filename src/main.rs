use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db_queries;
mod get_game_cards;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut address: String = "127.0.0.1:".to_owned();
    let port: String = utils::env_get("PORT".to_owned());
    address.push_str(&port);
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().allowed_origin("http://localhost:3000").finish())
            .route(
                "/api/get_game_cards",
                web::get().to(get_game_cards::get_cards),
            )
    })
    .workers(4) // Multi-threading(by default this number is equal to the number of logical CPUs in the system)
    .bind(address)?
    .run()
    .await
}
