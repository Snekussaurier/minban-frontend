use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct IdResponse {
    pub id: String,
}