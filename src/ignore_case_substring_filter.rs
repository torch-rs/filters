extern crate search_candidate;

use Filter;
use self::search_candidate::Key;
use self::search_candidate::SearchCandidate;

pub struct IgnoreCaseSubstringFilter;

impl Filter for IgnoreCaseSubstringFilter {

    fn filter<'s>(candidates: Vec<(SearchCandidate<'s>)>, search_term: &str) -> Vec<(SearchCandidate<'s>)> {
        let search_term = &search_term.to_lowercase();
        let mut filtered_candidates = Vec::new();
        for search_candidate in candidates {
            let candidate = search_candidate.get_value(Key::DisplayText);
            let lower_case_candidate = candidate.to_lowercase();
            if lower_case_candidate.contains(search_term) {
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
    use ignore_case_substring_filter::IgnoreCaseSubstringFilter;
    use self::search_candidate::Key;
    use self::search_candidate::SearchCandidate;

    #[test]
    fn ignore_case_substring_filter() {
        let sample_search_candidates = vec![
            SearchCandidate::new("ab", "ab", ""),
            SearchCandidate::new("bc", "Bc", ""),
            SearchCandidate::new("cd", "cd", ""),
            SearchCandidate::new("de", "de", ""),
            SearchCandidate::new("B", "b", ""),
        ];

        let filtered_candidates = IgnoreCaseSubstringFilter::filter(sample_search_candidates, "b");
        let actual_display_terms = vec!["ab", "Bc", "b"];
        for i in 0..filtered_candidates.len() {
            assert_eq!(filtered_candidates[i].get_value(Key::DisplayText), actual_display_terms[i]);
        }
    }

}
