use crate::models::{DefaultLink, SearchResult};
use std::fs;

pub fn find_default_results(query: &str) -> Vec<SearchResult> {
    let data = fs::read_to_string("../default_links.json")
        .unwrap_or("[]".to_string());

    let defaults: Vec<DefaultLink> =
        serde_json::from_str(&data).unwrap_or(vec![]);

    defaults
        .into_iter()
        .filter(|item| {
            item.keyword
                .to_lowercase()
                .contains(&query.to_lowercase())
        })
        .map(|item| SearchResult {
            title: item.title,
            url: item.url,
            snippet: "Priority result".to_string(),
            source: "Default".to_string(),
            score: item.priority,
        })
        .collect()
}