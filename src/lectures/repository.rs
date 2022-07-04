use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};

use super::entities::Lecture;
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
    pub fn lectures(&mut self) -> &[Lecture] {
        if self.cache.lectures.is_empty() {
            self.cache.lectures = self.scraper.fetch_lecture_details(None)
        }

        &self.cache.lectures
    }

    /// Attempts to load cached lecture information from a JSON file
    pub fn load_cache<P: AsRef<Path>>(&mut self, path: &P) -> std::io::Result<()> {
        let file = open_cache(path)?;
        self.cache = serde_json::from_reader(file)?;
        Ok(())
    }

    /// Serializes cache to JSON and writes it to a file
    pub fn save_cache<P: AsRef<Path>>(&self, path: &P) -> std::io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer(file, &self.cache)?;
        Ok(())
    }

    /// Serializes cache to JSON formatted with "pretty"-option and writes it to a file
    pub fn save_cache_pretty<P: AsRef<Path>>(&self, path: &P) -> std::io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer_pretty(file, &self.cache)?;
        Ok(())
    }

    /// Deletes cached lectures to force scraping them again on next getter call.
    pub fn invalidate_cache(&mut self) {
        self.cache = LectureCache::new();
    }
}

fn create_cache<P: AsRef<Path>>(path: &P) -> std::io::Result<File> {
    create_parent_directory(path)?;
    File::create(ensure_extension(path, "json"))
}

fn open_cache<P: AsRef<Path>>(path: &P) -> std::io::Result<File> {
    create_parent_directory(path)?;
    File::open(ensure_extension(path, "json"))
}

fn create_parent_directory<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    if let Some(directories) = path.as_ref().parent() {
        fs::create_dir_all(directories)?;
    }
    Ok(())
}

fn ensure_extension<P: AsRef<Path>>(path: &P, extension: &str) -> Box<Path> {
    let mut buf = path.as_ref().to_path_buf();
    buf.set_extension(extension);
    buf.into_boxed_path()
}