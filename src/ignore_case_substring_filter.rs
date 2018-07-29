extern crate search_candidate;

use Filter;
use self::search_candidate::SearchCandidate;
use self::search_candidate::Key;

pub struct IgnoreCaseSubstringFilter;

impl Filter for IgnoreCaseSubstringFilter {

    fn filter(candidates: Vec<SearchCandidate>, search_term: &str) -> Vec<SearchCandidate> {
        let search_term = &search_term.to_lowercase();
        let mut filtered_candidates = Vec::new();
        for search_candidate in &candidates {
            let candidate = search_candidate.get_value(Key::DisplayText);
            let lower_case_candidate = candidate.to_lowercase();
            if lower_case_candidate.contains(search_term) {
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
    use ignore_case_substring_filter::IgnoreCaseSubstringFilter;
    use self::search_candidate::SearchCandidate;
    use self::search_candidate::Key;

    #[test]
    fn ignore_case_substring_filter() {
        let sample_search_candidates = vec![
            SearchCandidate::new(String::from("ab"), String::from("ab"), String::from("")),
            SearchCandidate::new(String::from("bc"), String::from("Bc"), String::from("")),
            SearchCandidate::new(String::from("cd"), String::from("cd"), String::from("")),
            SearchCandidate::new(String::from("de"), String::from("de"), String::from("")),
            SearchCandidate::new(String::from("B"), String::from("b"), String::from("")),
        ];

        let filtered_candidates = IgnoreCaseSubstringFilter::filter(sample_search_candidates.clone(), "b");
        let actual_display_terms = vec!["ab", "Bc", "b"];
        for i in 0..filtered_candidates.len() {
            assert_eq!(filtered_candidates[i].get_value(Key::DisplayText), actual_display_terms[i]);
        }
    }

}
