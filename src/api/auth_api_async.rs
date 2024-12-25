use crate::api::routes::{ BASE_API_URL, API_VERSION };
use crate::mods::LoginRequest;
use crate::utils::LoginState;

pub async fn login(username: String, password: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}{}", BASE_API_URL, API_VERSION, "login");

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

pub async fn logout() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;
    
    let url = format!("{}{}", BASE_API_URL, "api/v1/logout");

    let _response = client
        .post(url)
        .fetch_credentials_include()
        .send()
        .await?
        .error_for_status()?;
    
    Ok(())
}

pub async fn check_auth() -> Result<LoginState, reqwest::Error> {
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