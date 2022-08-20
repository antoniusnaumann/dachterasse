use crate::asynch::datasource::{LoadResult, ReadOnlyDataSource};
use crate::asynch::scrape::LectureScraper;
use crate::StaticDegree;
use async_trait::async_trait;

#[derive(Default)]
pub struct ScraperSource {
    scraper: LectureScraper,
}

impl ScraperSource {
    pub fn new() -> Self {
        ScraperSource {
            scraper: LectureScraper::new(),
        }
    }
}

#[async_trait]
impl ReadOnlyDataSource for ScraperSource {
    async fn load_lectures(&self, degree: &'static StaticDegree) -> LoadResult {
        self.scraper
            .fetch_lecture_details(degree)
            .await
            .map_err(|err| err.to_string())
    }
}
