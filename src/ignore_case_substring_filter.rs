use Filter;

pub struct IgnoreCaseSubstringFilter;

impl Filter for IgnoreCaseSubstringFilter {

    fn filter(candidates: Vec<String>, search_term: String) -> Vec<String> {
        let search_term = search_term.to_lowercase();
        let mut filtered_candidates = Vec::new();
        for candidate in &candidates {
            let lower_case_candidate = candidate.to_lowercase();
            if lower_case_candidate.starts_with(search_term.as_str()) {
                filtered_candidates.push(candidate.to_string());
            }
        }
        filtered_candidates
    }
    
}

#[cfg(test)]
mod tests {

    use Filter;
    use ignore_case_substring_filter::IgnoreCaseSubstringFilter;

    #[test]
    fn ignore_case_substring_filter() {
        assert_eq!(IgnoreCaseSubstringFilter::filter(vec!["aBc".to_string(), "bc".to_string(),
                                                          "cdE".to_string(), "de".to_string(),
                                                          "abc".to_string()], "ab".to_string()),
                   ["aBc", "abc"]);
    }

}
