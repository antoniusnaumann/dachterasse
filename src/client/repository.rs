use crate::client::scrape::Lecture;
use super::scrape::LectureScraper;

#[derive(Default)]
pub struct LectureRepository {
    client: LectureScraper,
    lectures: Vec<Lecture>
}

impl LectureRepository {
    pub fn new() -> Self {
        LectureRepository { client: LectureScraper::new(), lectures: Vec::new() }
    }

    /// Getter method for lazily loading lectures from scraper.
    /// Subsequent method calls return cached lectures.
    pub fn lectures(&mut self) -> &Vec<Lecture> {
        if self.lectures.is_empty() {
            self.lectures = self.client.fetch_lecture_details(None)
        }

        &self.lectures
    }

    /// Deletes cached lectures to force scraping them again on next getter call.
    pub fn invalidate_cache(&mut self) {
        self.lectures = Vec::new();
    }
}