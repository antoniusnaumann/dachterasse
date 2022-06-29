use regex::Regex;
use reqwest::blocking::Client;
use crate::load;
use scraper::{Html, Selector};
use crate::load::load_lecture_html;

// TODO: Build lecture scraper which also follows links and scrapes categories
pub struct LectureScraper {
    client: Client
}

impl LectureScraper {
    pub fn new() -> LectureScraper {
        LectureScraper { client: Client::new() }
    }

    pub fn fetch_lectures(&self) -> Vec<(String, String)> {
        // TODO: Extract URL to enum for different degree tracks
        let url = "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html";
        let document = load_lecture_html(url, &self.client);
        let fragment = Html::parse_document(&document);
        let selector = Selector::parse("a.courselink").unwrap();
        let link_regex = Regex::new(r"/studium.*\.html").unwrap();

        fragment.select(&selector)
            .map(|element| {
                (String::from(
                    element.text()
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap()
                        .trim()),
                 String::from("hpi.de") + &link_regex.captures(&*element.html()).unwrap()[0])
            })
            .collect()
    }
}
