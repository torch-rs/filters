pub mod ignore_case_prefix_filter;
pub mod ignore_case_substring_filter;
pub mod prefix_filter;
pub mod substring_filter;

pub trait Filter {

    fn filter(candidates: Vec<String>, search_term: String) -> Vec<String>;

}
