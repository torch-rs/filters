extern crate search_candidate;

use Filter;
use self::search_candidate::Key;
use self::search_candidate::SearchCandidate;

pub struct SubstringFilter;

impl Filter for SubstringFilter {

    fn filter<'s>(candidates: Vec<(SearchCandidate<'s>)>, search_term: &str) -> Vec<(SearchCandidate<'s>)> {
        let mut filtered_candidates = Vec::new();
        for search_candidate in candidates {
            let candidate = search_candidate.get_value(Key::DisplayText);
            if candidate.contains(search_term) {
                filtered_candidates.push(search_candidate);
            }
        }
        filtered_candidates
    }
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Filter;
    use self::search_candidate::Key;
    use self::search_candidate::SearchCandidate;
    use substring_filter::SubstringFilter;

    #[test]
    fn substring_filter() {
        let sample_search_candidates = vec![
            SearchCandidate::new("ab", "ab", ""),
            SearchCandidate::new("bc", "bc", ""),
            SearchCandidate::new("cd", "cd", ""),
            SearchCandidate::new("de", "de", ""),
            SearchCandidate::new("b", "b", ""),
        ];

        let filtered_candidates = SubstringFilter::filter(sample_search_candidates, "b");
        let actual_display_terms = vec!["ab", "bc", "b"];
        for i in 0..filtered_candidates.len() {
            assert_eq!(filtered_candidates[i].get_value(Key::DisplayText), actual_display_terms[i]);
        }
    }

}
