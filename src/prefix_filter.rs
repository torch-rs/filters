use Filter;

pub struct PrefixFilter;

impl Filter for PrefixFilter {

    fn filter(candidates: Vec<String>, search_term: String) -> Vec<String> {
        let mut filtered_candidates = Vec::new();
        for candidate in &candidates {
            if candidate.starts_with(search_term.as_str()) {
                filtered_candidates.push(candidate.to_string());
            }
        }
        filtered_candidates
    }
    
}

#[cfg(test)]
mod tests {

    use Filter;
    use prefix_filter::PrefixFilter;

    #[test]
    fn prefix_filter() {
        assert_eq!(PrefixFilter::filter(vec!["ab".to_string(), "bc".to_string(),
                                             "cd".to_string(), "de".to_string()], "ab".to_string()), ["ab"]);
    }

}
