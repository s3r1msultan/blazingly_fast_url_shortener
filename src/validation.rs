use regex::Regex;

pub fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(
        r"^(https?://)?([a-zA-Z0-9.-]+)(\.[a-zA-Z]{2,6})(/[^\s]*)?$"
    ).unwrap();

    url_regex.is_match(url)
}