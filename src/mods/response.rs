use serde::Deserialize;
use std::collections::BTreeSet;
use std::collections::HashMap;

use super::{CardModel, StateModel, TagModel};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Clone, PartialEq)]
pub struct FetchResponse {
    pub cards: HashMap<u32, BTreeSet<CardModel>>,
    pub columns: Vec<StateModel>,
    pub tags: Vec<TagModel>,
}
