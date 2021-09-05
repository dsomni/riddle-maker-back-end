use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameCard {
  pub game_id: i32,
  pub title: String,
  pub info: String,
  pub img_url: String,
  pub page_url: String,
}

pub fn connect_to_db() -> Client {
  let client = Client::connect(
    "host=localhost port=6432 user=riddlemaker password=1974",
    NoTls,
  ); // connect

  // Error checking
  match client {
    Ok(client) => return client,
    Err(e) => panic!("Connection Error: \n {}", e),
  }
}
pub fn get_games_vec(client: &mut Client) -> Vec<GameCard> {
  // Query and error checking
  let rows = match client.query("SELECT * FROM games", &[]) {
    Ok(games) => games,
    Err(e) => panic!("Error on query: \n{}", e),
  };

  // Forming games array
  let mut games = Vec::<GameCard>::new();
  for row in rows {
    games.push(GameCard {
      game_id: row.get::<_, i32>(0),
      title: row.get::<_, String>(1),
      info: row.get::<_, String>(2),
      img_url: row.get::<_, String>(3),
      page_url: row.get::<_, String>(4),
    })
  }
  games
}
