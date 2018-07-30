extern crate search_candidate;

pub mod ignore_case_prefix_filter;
pub mod ignore_case_substring_filter;
pub mod prefix_filter;
pub mod substring_filter;

use self::search_candidate::SearchCandidate;

pub trait Filter {

    fn filter<'s>(candidates: Vec<(SearchCandidate<'s>)>, search_term: &str) -> Vec<(SearchCandidate<'s>)>;
    
}
