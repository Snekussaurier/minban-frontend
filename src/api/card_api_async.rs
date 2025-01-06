use std::collections::{BTreeSet, HashMap};

use crate::mods::CardModel;
use crate::api::routes::{ BASE_API_URL, API_VERSION };
use crate::IdResponse;
use dioxus::logger::tracing::debug;

pub async fn get_cards() -> Result<HashMap<u32, BTreeSet<CardModel>>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "cards");

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let cards: Vec<CardModel> = response.json().await?;
    
    let mut board: HashMap<u32, BTreeSet<CardModel>> = HashMap::new();

    for card in cards {
        board.entry(card.state_id)
            .or_insert_with(BTreeSet::new)
            .insert(card);
    }

    debug!("cards: {:?}", board);

    Ok(board)
}

pub async fn create_card(card: &CardModel) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "card");

    let response = client
        .post(url)
        .json(card)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let id_response: IdResponse = response.json().await?;
    
    Ok(id_response.id)
}

pub async fn patch_card(card: &CardModel) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "card/", card.id);

    let _response = client
        .patch(url)
        .json(card)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

pub async fn delete_card(card_id: &String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "card/", card_id);

    let _response = client
        .delete(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}