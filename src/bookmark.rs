use serde::{Deserialize, Serialize};
// use std::error;
// use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub url: String,
    pub description: Option<String>,
    pub collection: Option<Vec<String>>,
    pub tags: Vec<String>,
}
