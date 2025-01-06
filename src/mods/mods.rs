use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CardModel {
    #[serde(skip_serializing)]
    pub id: String,
    pub title: String,
    pub description: String,
    pub position: u32,
    pub state_id: u32,
    pub tags: Vec<TagModel>,
}

impl Ord for CardModel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for CardModel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.position.cmp(&other.position))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StateModel {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub position: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TagModel {
    pub id: u32,
    pub name: String,
    pub color: String,
}