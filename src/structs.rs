use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseObj {
  pub status: u8,
  pub game_cards: Vec<GameCard>,
}

#[derive(Serialize, Deserialize)]
pub struct GameCard {
  pub game_id: i32,
  pub title: String,
  pub info: String,
  pub img_url: String,
  pub page_url: String,
}
