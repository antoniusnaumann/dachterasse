use regex::Regex;
use crate::load;
use scraper::{Html, Selector};

// TODO: Build lecture scraper which also follows links and scrapes categories

pub fn fetch_lectures() -> Vec<(String, String)> {
    let url = "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html";
    let document = load::load_lecture_html(&url);
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
