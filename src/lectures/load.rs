use reqwest::blocking::Client;

pub fn get_text(url: &str, client: &Client) -> Result<String, reqwest::Error> {
    client
        .get(url)
        .send()?
        .text()
}