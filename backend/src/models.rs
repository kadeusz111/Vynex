use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {

    pub title: String,

    pub url: String,

    pub snippet: String,

    pub source: String,

    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultLink {

    pub keyword: String,

    pub title: String,

    pub url: String,

    pub priority: i32,
}