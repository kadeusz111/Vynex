use crate::models::SearchResult;

pub fn rank_results(query: &str, results: &mut Vec<SearchResult>) {

    for result in results.iter_mut() {

        let mut score = result.score;

        let title = result.title.to_lowercase();
        let query = query.to_lowercase();

        if title.contains(&query) {
            score += 50;
        }

        if result.url.contains(&query) {
            score += 25;
        }

        result.score = score;
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));
}