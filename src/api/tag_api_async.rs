use crate::mods::StateModel;
use crate::mods::IdResponse;
use crate::api::routes::{ BASE_API_URL, API_VERSION };
use dioxus_logger::tracing::debug;

pub async fn get_tags() -> Result<Vec<StateModel>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "tags");

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let tags: Vec<StateModel> = response.json().await?;
    
    debug!("tags: {:?}", tags);

    Ok(tags)
}

pub async fn create_tag(tag: StateModel) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "tag");

    let response = client
        .post(url)
        .json(&tag)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    let id_response: IdResponse = response.json().await?;
    
    Ok(id_response.id)
}

pub async fn patch_tag(tag: StateModel) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "tag/", &tag.id);

    let _response = client
        .patch(url)
        .json(&tag)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn delete_tag(tag: StateModel) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}{}", BASE_API_URL, API_VERSION, "tag/", &tag.id);

    let _response = client
        .delete(url)
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}