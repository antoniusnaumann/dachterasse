use super::scrape::LectureScraper;

#[derive(Default)]
pub struct LectureRepository {
    client: LectureScraper
}

impl LectureRepository {
    pub fn new() -> Self {
        LectureRepository { client: LectureScraper::new() }
    }
}