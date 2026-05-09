mod defaults;
mod models;
mod ranking;
mod search;

use axum::{
    extract::{
        Query,
        State,
    },
    response::Json,
    routing::get,
    Router,
};

use serde::Deserialize;

use std::sync::Arc;

use tokio::sync::Semaphore;

use tower_http::cors::CorsLayer;

use defaults::find_default_results;

use models::SearchResult;

use ranking::rank_results;

use search::{
    scrape_duckduckgo,
    scrape_github,
    scrape_invidious,
    scrape_reddit,
    scrape_wikipedia,
};

#[derive(Clone)]
struct AppState {

    semaphore: Arc<Semaphore>,
}

#[derive(Deserialize)]
struct SearchQuery {

    q: String,
}

#[tokio::main]
async fn main() {

    let state = AppState {

        semaphore: Arc::new(
            Semaphore::new(10)
        ),
    };

    let app = Router::new()

        .route(
            "/search",
            get(search_handler)
        )

        .layer(
            CorsLayer::permissive()
        )

        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(
            "0.0.0.0:3000"
        )
        .await
        .unwrap();

    println!(
        "Server running on http://localhost:3000"
    );

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn search_handler(

    State(state): State<AppState>,

    Query(params): Query<SearchQuery>,

) -> Json<Vec<SearchResult>> {

    let _permit = state
        .semaphore
        .acquire()
        .await
        .unwrap();

    let query = params.q;

    let mut results = Vec::new();


    let mut default_results =
        find_default_results(&query);

    results.append(&mut default_results);


    let (

        wiki_results,

        github_results,

        reddit_results,

        youtube_results,

        ddg_results

    ) = tokio::join!(

        scrape_wikipedia(&query),

        scrape_github(&query),

        scrape_reddit(&query),

        scrape_invidious(&query),

        scrape_duckduckgo(&query)
    );



    results.extend(wiki_results);

    results.extend(github_results);

    results.extend(reddit_results);

    results.extend(youtube_results);

    // DUCKDUCKGO NA SAMYM DOLE
    results.extend(ddg_results);


    rank_results(
        &query,
        &mut results
    );


    let mut seen =
        std::collections::HashSet::new();

    results.retain(|item| {
        seen.insert(item.url.clone())
    });

    Json(results)
}