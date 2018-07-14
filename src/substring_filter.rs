use Filter;

pub struct SubstringFilter;

impl Filter for SubstringFilter {

    fn filter(candidates: Vec<String>, search_term: String) -> Vec<String> {
        let mut filtered_candidates = Vec::new();
        for candidate in &candidates {
            if candidate.contains(search_term.as_str()) {
                filtered_candidates.push(candidate.to_string());
            }
        }
        filtered_candidates
    }
    
}

#[cfg(test)]
mod tests {

    use Filter;
    use substring_filter::SubstringFilter;

    #[test]
    fn substring_filter() {
        assert_eq!(SubstringFilter::filter(vec!["ab".to_string(), "bc".to_string(),
                                                "cd".to_string(), "de".to_string()], "b".to_string()), ["ab", "bc"]);
    }

}
