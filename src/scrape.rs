use crate::load;
use scraper::{Html, Selector};

// TODO: Build lecture scraper which also follows links and scrapes categories

pub fn fetch_lectures() -> Vec<String> {
    let url = "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html";
    let document = load::load_lecture_html(&url);
    let fragment = Html::parse_document(&document);
    let selector = Selector::parse("a.courselink").unwrap();

    fragment.select(&selector)
        .map(|element| {
            String::from(
                element.text()
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .trim())
        })
        .collect()
}
