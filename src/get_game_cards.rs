use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db_queries::{connect_to_db, get_games_vec, GameCard};

#[derive(Serialize, Deserialize)]
struct ResponseObj {
    pub status: u8,
    pub game_cards: Vec<GameCard>,
}

pub async fn get_cards() -> impl Responder {
    let mut client = connect_to_db();
    let game_cards = get_games_vec(&mut client);
    HttpResponse::Ok().json(ResponseObj {
        status: 200,
        game_cards,
    })
}
