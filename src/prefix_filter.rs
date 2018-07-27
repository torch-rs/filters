extern crate search_candidate;

use Filter;
use self::search_candidate::SearchCandidate;
use self::search_candidate::Key;

pub struct PrefixFilter;

impl Filter for PrefixFilter {

    fn filter(candidates: Vec<SearchCandidate>, search_term: String) -> Vec<SearchCandidate> {
        let mut filtered_candidates = Vec::new();
        for search_candidate in &candidates {
            let candidate = search_candidate.get_value(Key::DisplayText);
            if candidate.starts_with(search_term.as_str()) {
                filtered_candidates.push(search_candidate.clone());
            }
        }
        filtered_candidates
    }
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Filter;
    use prefix_filter::PrefixFilter;
    use self::search_candidate::SearchCandidate;
    use self::search_candidate::Key;

    #[test]
    fn prefix_filter() {
        let sample_search_candidates = vec![
            SearchCandidate::new(String::from("ab"), String::from("ab"), String::from("")),
            SearchCandidate::new(String::from("bc"), String::from("bc"), String::from("")),
            SearchCandidate::new(String::from("cd"), String::from("cd"), String::from("")),
            SearchCandidate::new(String::from("de"), String::from("de"), String::from("")),
            SearchCandidate::new(String::from("b"), String::from("b"), String::from("")),
        ];

        let filtered_candidates = PrefixFilter::filter(sample_search_candidates.clone(), String::from("b"));
        let actual_display_terms = vec!["bc", "b"];
        for i in 0..filtered_candidates.len() {
            assert_eq!(filtered_candidates[i].get_value(Key::DisplayText), actual_display_terms[i]);
        }
    }

}
