use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::app_index::AppEntry;

/// Return the top N fuzzy-matched apps for the given query.
pub fn fuzzy_search<'a>(
    query: &str,
    apps: &'a [AppEntry],
    limit: usize,
) -> Vec<(i64, &'a AppEntry)> {
    if query.is_empty() {
        // Return first `limit` apps sorted by name when no query
        let mut results: Vec<(i64, &AppEntry)> = apps.iter().map(|a| (0, a)).collect();
        results.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
        results.truncate(limit);
        return results;
    }

    let matcher = SkimMatcherV2::default();
    let mut results: Vec<(i64, &AppEntry)> = apps
        .iter()
        .filter_map(|app| {
            // Match against name primarily, also check description
            let name_score = matcher.fuzzy_match(&app.name, query);
            let desc_score = app
                .description
                .is_empty()
                .then_some(None)
                .unwrap_or_else(|| matcher.fuzzy_match(&app.description, query));

            let score = match (name_score, desc_score) {
                (Some(n), Some(d)) => Some(n.max(d / 2)),
                (Some(n), None) => Some(n),
                (None, Some(d)) => Some(d / 2),
                (None, None) => None,
            };

            score.map(|s| (s, app))
        })
        .collect();

    results.sort_by(|a, b| b.0.cmp(&a.0));
    results.truncate(limit);
    results
}
