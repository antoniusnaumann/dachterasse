use reqwest::blocking::Client;

pub fn unsafe_get(url: &str, client: &Client) -> String {
    client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap()
}