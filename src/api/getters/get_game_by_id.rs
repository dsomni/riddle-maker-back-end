use actix_web::{web, HttpResponse, Responder};

use crate::db_queries::{connect_to_db, get_games_by_key};
use crate::structs::ResponseObj;

pub async fn get_cards_by_id(id: web::Path<i32>) -> impl Responder {
  let mut client = connect_to_db();
  let game_cards = get_games_by_key(
    &mut client,
    "games",
    "gameid",
    &id.to_string(),
  );

  HttpResponse::Ok().json(ResponseObj {
    status: 200,
    game_cards,
  })
}
