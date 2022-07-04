use super::repository::LectureRepository;
use super::scrape::Lecture;

#[derive(Default)]
pub struct LectureClient {
    repository: LectureRepository
}

impl LectureClient {
    pub fn new() -> Self {
        let mut client = LectureClient { repository: LectureRepository::new() };
        if let Err(e) = client.repository.load_cache() {
            eprintln!("Could not load cache: {}", e);
        }
        client
    }

    /// Loads lectures from repository. Repository might load lectures lazily on first attempt.
    pub fn lectures(&mut self) -> &Vec<Lecture> {
        self.repository.lectures()
    }
}

impl Drop for LectureClient {
    fn drop(&mut self) {
        if let Err(e) = self.repository.save_cache() {
            eprintln!("Could not save cache: {}", e);
        }
    }
}