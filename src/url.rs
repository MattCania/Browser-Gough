use crate::types::structs;

// pub fn parse(url: &str) -> structs::Url {
pub fn parse(url: &str) {
    let mut scheme = String::new();
    let subdomain = String::new();
    let domain = String::new();
    let top_level_domain = String::new();
    let port = String::new();
    let path = String::new();
    let query_separator = String::new();
    let query_string = String::new();
    let fragment = String::new();

    println!("Parsing: {}", url);

    if let Some((s, _)) = url.split_once("://") {
        scheme = s.to_string();
    }
    
    if scheme != "http" && scheme != "https" {
        println!("Invalid Scheme!");
        return; // Supposedly returns an invalid response
    }

    let split = url.split("/");
    for parts in split.filter(|parts| !parts.is_empty()) {
        println!("{}", parts)
    }
}