use crate::datasource::{LoadResult, ReadOnlyDataSource};
use crate::Degree;
use crate::scrape::LectureScraper;

#[derive(Default)]
pub struct ScraperSource {
    scraper: LectureScraper
}

impl ScraperSource {
    pub fn new() -> Self {
        ScraperSource { scraper: LectureScraper::new() }
    }
}

impl ReadOnlyDataSource for ScraperSource {
    fn load_lectures(&self, degree: &'static Degree) -> LoadResult {
        self.scraper
            .fetch_lecture_details(degree)
            .map_err(|err| err.to_string())
    }
}