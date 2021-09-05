use postgres::{Client, NoTls};

use crate::structs::GameCard;

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
  form_games_vec(&rows)
}

// TODO: generic for value type
pub fn get_games_by_key(client: &mut Client, db: String, key: String, value: String) -> Vec<GameCard> {
  let query = format!("SELECT * FROM {} WHERE {} = {}", db, key, value);
  let rows = match client.query(query.as_str(), &[]) {
    Ok(games) => games,
    Err(e) => panic!("Error on query by key: \n{}", e),
  };
  form_games_vec(&rows)
}

fn form_games_vec(rows: &Vec<postgres::Row>) -> Vec<GameCard> {
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
