use std::str;
use dioxus_logger::tracing::info;
use serde::{ Deserialize, Serialize };

use crate::LoginState;

pub static BASE_API_URL: &str = "http://localhost:9916/";
pub static LOGIN_API: &str = "api/v1/login";

pub async fn login(username: String, password: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}", BASE_API_URL, LOGIN_API);

    let _response = client
        .post(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .json(&LoginRequest {
            username: username,
            password: password,
        })
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

pub async fn checkAuth() -> Result<LoginState, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}", BASE_API_URL, "api/v1/check-auth");

    let response = client
        .get(url)
        .fetch_credentials_include()
        .send()
        .await?;

    if response.status().as_u16() == 401 {
        return Ok(LoginState::NotLoggedIn);
    }
    
    Ok(LoginState::LoggedIn)
}

pub async fn get_cards() -> Result<Vec<Card>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}", BASE_API_URL, "api/v1/cards");

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let cards: Vec<Card> = response.json().await?;
    
    info!("cards: {:?}", cards);

    Ok(cards)
}

pub async fn patch_card(card: Card) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, "api/v1/card/", &card.id);

    let _response = client
        .patch(url)
        .json(&card)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

pub async fn get_states() -> Result<Vec<State>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}", BASE_API_URL, "api/v1/states");

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let states: Vec<State> = response.json().await?;
    
    info!("states: {:?}", states);

    Ok(states)
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Tag {
    id: u32,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Card {
    pub id: String,
    pub title: String,
    pub description: String,
    pub position: u32,
    pub state_id: u32,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct State {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub position: u32,
}