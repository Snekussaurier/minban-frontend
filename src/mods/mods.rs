use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct CardModel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub position: u32,
    pub state_id: u32,
    pub tags: Vec<TagModel>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StateModel {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub position: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TagModel {
    id: u32,
    pub name: String,
    pub color: String,
}