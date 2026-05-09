use dashmap::DashMap;
use crate::models::SearchResult;

pub type SearchCache = DashMap<String, Vec<SearchResult>>;