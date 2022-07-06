use std::collections::HashMap;
use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use scraper::element_ref::Text;
use super::load::get_text;
use super::entities::Lecture;

// TODO: Build lecture scraper which also follows links and scrapes categories
#[derive(Default)]
pub struct LectureScraper {
    client: Client
}

pub type Error = reqwest::Error;

impl LectureScraper {
    pub fn new() -> Self {
        LectureScraper { client: Client::new() }
    }

    // TODO: Extract document fetching and parsing into separate method for better testability
    pub fn fetch_lectures(&self) -> Result<Vec<Lecture>, Error> {
        // TODO: Extract URL to enum for different degree tracks
        let url = "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html";
        let document = get_text(url, &self.client)?;
        let fragment = Html::parse_document(&document);
        let selector = Selector::parse("a.courselink").unwrap();
        let link_regex = Regex::new(r"/studium.*\.html").unwrap();

        Ok(fragment.select(&selector)
            .map(|element| {
                Lecture {
                    title: clean(element.text()),
                    url: String::from("https://hpi.de") + &link_regex.captures(&*element.html()).unwrap()[0],
                    categories: None
                }
            })
            .collect())
    }

    // TODO: Extract document fetching and parsing into separate method for better testability
    pub fn fetch_lecture_details(&self, lectures: Option<Vec<Lecture>>) -> Result<Vec<Lecture>, Error> {
        let mut lectures = if let Some(vector) = lectures { vector } else { self.fetch_lectures()? };

        for mut lecture in &mut lectures {
            // TODO: Do asynchronously
            let document = get_text(&lecture.url, &self.client)?;

            if let Some(inner_fragment) = self.scrape_modules(document.as_str(), "IT-Systems Engineering MA") {
                let module_list = Html::parse_fragment(inner_fragment.as_str());
                let item_selector = Selector::parse("li").unwrap();
                let categories: Vec<(String, Vec<String>)> = module_list.select(&item_selector)
                    .map(|element| {
                        (clean(element.text()),
                         Html::parse_fragment(element.inner_html().as_str())
                             .select(&item_selector)
                             .map(|child| clean(child.text()))
                             .collect::<Vec<_>>())
                    })
                    .filter(|(_, children)| !children.is_empty())
                    .collect();

                let mut category_map = HashMap::<String, Vec<String>>::new();
                for mut category in categories {
                    category_map.entry(category.0).or_insert(Vec::new()).append(&mut category.1);
                }
                lecture.categories = Some(category_map);
            }
        }

        Ok(lectures)
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

fn clean(text: Text) -> String {
    String::from(clean_str(text))
}

fn clean_str(text: Text) -> &str {
    text.collect::<Vec<_>>()
        .first()
        .unwrap()
        .trim()
}
