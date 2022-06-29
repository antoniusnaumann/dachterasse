use reqwest::blocking::Client;

pub fn load_lecture_html(url: &str, client: &Client) -> String {
    client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap()
}