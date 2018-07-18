use Filter;

pub struct IgnoreCasePrefixFilter;

impl Filter for IgnoreCasePrefixFilter {

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
    use ignore_case_prefix_filter::IgnoreCasePrefixFilter;

    #[test]
    fn prefix_filter() {
        assert_eq!(IgnoreCasePrefixFilter::filter(vec!["Ab".to_string(), "bc".to_string(),
                                                       "cd".to_string(), "De".to_string()], "ab".to_string()), ["Ab"]);
    }

}
