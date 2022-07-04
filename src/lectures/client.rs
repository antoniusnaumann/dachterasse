use super::repository::LectureRepository;
use super::scrape::Lecture;
use super::config::Config;

pub struct LectureClient {
    repository: LectureRepository,
    config: Config
}

impl LectureClient {
    pub fn with_config(config: Config) -> Self {
        let mut client = LectureClient { repository: LectureRepository::new(), config };
        if let Some(path) = client.config.get_cache_path() {
            if let Err(e) = client.repository.load_cache(&path) {
                eprintln!("Could not load cache: {}", e);
            }
        }
        client
    }

    // TODO: Convert to async function
    /// Loads lectures from repository. Repository might load lectures lazily on first attempt, so this could take a while.
    pub fn lectures(&mut self) -> &Vec<Lecture> {
        self.repository.lectures()
    }
}

impl Drop for LectureClient {
    fn drop(&mut self) {
        if let Some(path) = self.config.get_cache_path() {
            if let Err(e) = self.repository.save_cache(&path.to_string()) {
                eprintln!("Could not save cache: {}", e);
            }
        }
    }
}