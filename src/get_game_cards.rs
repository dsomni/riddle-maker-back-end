use actix_web::{ HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GameCard {
    pub title: String,
    pub info: String,
    pub img_url: String,
    pub page_url: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseObj {
    pub status: u8,
    pub game_cards: Vec::<GameCard>,
}

pub async fn get_cards() -> impl Responder {
    let mut game_cards = Vec::<GameCard>::new();
    game_cards.push(GameCard {
        title: "LOL1".to_owned(),
        info: "KEK1".to_owned(),
        img_url: "sdgffdgf1".to_owned(),
        page_url: "game1".to_owned()
    });
    game_cards.push(GameCard {
        title: "vsdvdf".to_owned(),
        info: "fvfdvdfvfd".to_owned(),
        img_url: "vdfbfgbfgbgf".to_owned(),
        page_url: "gadfgfdgfdgfdgfdgfme".to_owned()
    });
    game_cards.push(GameCard {
        title: "LOL3".to_owned(),
        info: "KEK3".to_owned(),
        img_url: "sdgffdgf3".to_owned(),
        page_url: "game3".to_owned()
    });

    HttpResponse::Ok().json(ResponseObj {
        status: 200,
        game_cards
    })
}