use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use scraper::element_ref::Text;
use super::load::unsafe_get;

// TODO: Build lecture scraper which also follows links and scrapes categories
#[derive(Default)]
pub struct LectureScraper {
    client: Client
}

pub struct Lecture {
    pub title: String,
    pub url: String,
    // pub description: Option<String>,
    pub categories: Option<Vec<String>>,
    // TODO: pub degrees: Vec<Degree> where Degree is an enum containing e.g. ITSE-BA, ITSE-MA and so on
}

impl LectureScraper {
    pub fn new() -> Self {
        LectureScraper { client: Client::new() }
    }

    // TODO: Extract document fetching and parsing into separate method for better testability
    pub fn fetch_lectures(&self) -> Vec<Lecture> {
        // TODO: Extract URL to enum for different degree tracks
        let url = "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html";
        let document = unsafe_get(url, &self.client);
        let fragment = Html::parse_document(&document);
        let selector = Selector::parse("a.courselink").unwrap();
        let link_regex = Regex::new(r"/studium.*\.html").unwrap();

        fragment.select(&selector)
            .map(|element| {
                Lecture {
                    title: String::from(clean(element.text())),
                    url: String::from("https://hpi.de") + &link_regex.captures(&*element.html()).unwrap()[0],
                    categories: None
                }
            })
            .collect()
    }

    // TODO: Extract document fetching and parsing into separate method for better testability
    pub fn fetch_lecture_details(&self, lectures: Option<Vec<Lecture>>) -> Vec<Lecture> {
        let mut lectures = if let Some(vector) = lectures { vector } else { self.fetch_lectures() };

        for mut lecture in &mut lectures {
            // TODO: Do asynchronously
            let document = unsafe_get(&lecture.url, &self.client);

            if let Some(inner_fragment) = self.scrape_modules(document.as_str(), "IT-Systems Engineering MA") {
                let category_list = Html::parse_fragment(inner_fragment.as_str());
                let category_selector = Selector::parse("li").unwrap();
                lecture.categories = Some(category_list.select(&category_selector)
                    .map(|element| String::from(clean(element.text())))
                    .collect()
                );
            }
        }

        lectures
    }

    fn scrape_modules(&self, document: &str, degree: &str) -> Option<String> {
        let fragment = Html::parse_document(document);
        let selector = Selector::parse("div.tx_dscclipclap").unwrap();
        let header_selector = Selector::parse("div.tx_dscclipclap_header").unwrap();

        fragment.select(&selector)
            .find(|element| {
                Html::parse_fragment(element.inner_html().as_str())
                    .select(&header_selector)
                    .any(|inner| clean(inner.text()) == degree)
            })
            .map(|optional| optional.inner_html())
    }
}

fn clean(text: Text) -> &str {
    text.collect::<Vec<_>>()
        .first()
        .unwrap()
        .trim()
}
