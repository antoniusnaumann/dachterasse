use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};

use super::scrape::Lecture;
use super::scrape::LectureScraper;

#[derive(Default, Serialize, Deserialize)]
struct LectureCache {
    lectures: Vec<Lecture>
}

impl LectureCache {
    fn new() -> Self {
        LectureCache { lectures: Vec::new() }
    }
}

#[derive(Default)]
pub struct LectureRepository {
    scraper: LectureScraper,
    cache: LectureCache,
}

impl LectureRepository {
    pub fn new() -> Self {
        LectureRepository { scraper: LectureScraper::new(), cache: LectureCache::new() }
    }

    /// Getter method for lazily loading lectures from scraper.
    /// Subsequent method calls return cached lectures.
    pub fn lectures(&mut self) -> &Vec<Lecture> {
        if self.cache.lectures.is_empty() {
            self.cache.lectures = self.scraper.fetch_lecture_details(None)
        }

        &self.cache.lectures
    }

    /// Attempts to load cached lecture information from a JSON file
    pub fn load_cache(&mut self) -> std::io::Result<()> {
        let file = &self.open_cache()?;
        self.cache = serde_json::from_reader(file)?;
        Ok(())
    }

    /// Serializes cache to JSON and writes it to a file
    pub fn save_cache(&self) -> std::io::Result<()> {
        let file = &self.create_cache()?;
        serde_json::to_writer(file, &self.cache)?;
        Ok(())
    }

    /// Serializes cache to JSON formatted with "pretty"-option and writes it to a file
    pub fn save_cache_pretty(&self) -> std::io::Result<()> {
        let file = &self.create_cache()?;
        serde_json::to_writer_pretty(file, &self.cache)?;
        Ok(())
    }

    fn create_cache(&self) -> std::io::Result<File> {
        fs::create_dir_all("cache")?;
        let path = Path::new("cache/lecture_cache.json");
        File::create(path)
    }

    fn open_cache(&self) -> std::io::Result<File> {
        fs::create_dir_all("cache")?;
        let path = Path::new("cache/lecture_cache.json");
        File::open(path)
    }

    /// Deletes cached lectures to force scraping them again on next getter call.
    pub fn invalidate_cache(&mut self) {
        self.cache = LectureCache::new();
    }
}