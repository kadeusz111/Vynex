use crate::models::SearchResult;

use scraper::{Html, Selector};

use reqwest::Client;

use std::time::Duration;

fn create_client() -> Client {

    Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
        )
        .build()
        .unwrap()
}

pub async fn scrape_wikipedia(
    query: &str
) -> Vec<SearchResult> {

    let client = create_client();

    let url = format!(
        "https://en.wikipedia.org/wiki/{}",
        query
    );

    let response = client
        .get(&url)
        .send()
        .await;

    let mut results = Vec::new();

    if let Ok(resp) = response {

        if resp.status().is_success() {

            results.push(SearchResult {
                title: format!(
                    "Wikipedia - {}",
                    query
                ),
                url,
                snippet: "Wikipedia article".to_string(),
                source: "Wikipedia".to_string(),
                score: 80,
            });
        }
    }

    results
}

pub async fn scrape_github(
    query: &str
) -> Vec<SearchResult> {

    let client = create_client();

    let url = format!(
        "https://github.com/search?q={}",
        query
    );

    let response = client
        .get(&url)
        .send()
        .await;

    let mut results = Vec::new();

    if let Ok(resp) = response {

        if let Ok(body) = resp.text().await {

            let document =
                Html::parse_document(&body);

            let selector =
                Selector::parse("a.v-align-middle")
                    .unwrap();

            for element in document
                .select(&selector)
                .take(5)
            {

                let title = element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();

                let href = element
                    .value()
                    .attr("href")
                    .unwrap_or("");

                let full_url = format!(
                    "https://github.com{}",
                    href
                );

                results.push(SearchResult {
                    title,
                    url: full_url,
                    snippet: "GitHub repository".to_string(),
                    source: "GitHub".to_string(),
                    score: 70,
                });
            }
        }
    }

    results
}

pub async fn scrape_reddit(
    query: &str
) -> Vec<SearchResult> {

    let client = create_client();

    let url = format!(
        "https://www.reddit.com/search/?q={}",
        query
    );

    let response = client
        .get(&url)
        .send()
        .await;

    let mut results = Vec::new();

    if let Ok(resp) = response {

        if let Ok(body) = resp.text().await {

            let document =
                Html::parse_document(&body);

            let selector =
                Selector::parse("a[data-click-id='body']")
                    .unwrap();

            for element in document
                .select(&selector)
                .take(5)
            {

                let title = element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();

                let href = element
                    .value()
                    .attr("href")
                    .unwrap_or("");

                let full_url = format!(
                    "https://reddit.com{}",
                    href
                );

                results.push(SearchResult {
                    title,
                    url: full_url,
                    snippet: "Reddit discussion".to_string(),
                    source: "Reddit".to_string(),
                    score: 60,
                });
            }
        }
    }

    results
}

pub async fn scrape_invidious(
    query: &str
) -> Vec<SearchResult> {

    let client = create_client();

    let url = format!(
        "https://yewtu.be/search?q={}",
        query
    );

    let response = client
        .get(&url)
        .send()
        .await;

    let mut results = Vec::new();

    if let Ok(resp) = response {

        if let Ok(body) = resp.text().await {

            let document =
                Html::parse_document(&body);

            let selector =
                Selector::parse("a[href^='/watch']")
                    .unwrap();

            for element in document
                .select(&selector)
                .take(5)
            {

                let title = element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();

                let href = element
                    .value()
                    .attr("href")
                    .unwrap_or("");

                let full_url = format!(
                    "https://yewtu.be{}",
                    href
                );

                results.push(SearchResult {
                    title,
                    url: full_url,
                    snippet: "YouTube video".to_string(),
                    source: "YouTube".to_string(),
                    score: 75,
                });
            }
        }
    }

    results
}

pub async fn scrape_duckduckgo(
    query: &str
) -> Vec<SearchResult> {

    let client = create_client();

    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        query
    );

    let response = client
        .get(url)
        .send()
        .await;

    let mut results = Vec::new();

    if let Ok(resp) = response {

        if let Ok(body) = resp.text().await {

            let document =
                Html::parse_document(&body);

            let selector =
                Selector::parse("a.result__a")
                    .unwrap();

            for element in document
                .select(&selector)
                .take(5)
            {

                let title = element
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();

                let url = element
                    .value()
                    .attr("href")
                    .unwrap_or("")
                    .to_string();

                results.push(SearchResult {
                    title,
                    url,
                    snippet: "DuckDuckGo result".to_string(),
                    source: "DuckDuckGo".to_string(),
                    score: 10,
                });
            }
        }
    }

    results
}