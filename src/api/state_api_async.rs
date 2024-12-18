use crate::mods::StateModel;
use crate::mods::IdResponse;
use crate::api::routes::{ BASE_API_URL, API_VERSION };
use dioxus::logger::tracing::debug;

pub async fn get_states() -> Result<Vec<StateModel>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "states");

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let states: Vec<StateModel> = response.json().await?;
    
    debug!("states: {:?}", states);

    Ok(states)
}

pub async fn create_state(state: StateModel) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "state");

    let response = client
        .post(url)
        .json(&state)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let id_response: IdResponse = response.json().await?;
    
    Ok(id_response.id)
}

pub async fn patch_state(state: StateModel) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "state/", &state.id);

    let _response = client
        .patch(url)
        .json(&state)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn delete_state(state: StateModel) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "state/", &state.id);

    let _response = client
        .delete(url)
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}