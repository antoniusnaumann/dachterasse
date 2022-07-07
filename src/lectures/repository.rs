use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::lectures::entities::{Degree, Degrees};

use super::entities::Lecture;
use super::scrape::LectureScraper;

pub use super::scrape::Error;

#[derive(Default, Serialize, Deserialize)]
struct LectureCache {
    lectures: Vec<Lecture>
}

impl LectureCache {
    fn new(lectures: Vec<Lecture>) -> Self {
        LectureCache { lectures }
    }
}

#[derive(Default)]
pub struct LectureRepository {
    scraper: LectureScraper,
    caches: HashMap<&'static Degree, LectureCache>,
}

impl LectureRepository {
    pub fn new() -> Self {
        LectureRepository { scraper: LectureScraper::new(), caches: HashMap::new() }
    }

    /// Get the curently cached lectures without loading them if not present.
    /// Returns an empty slice if no lectures are loaded.
    pub fn lectures(&self, degree: &'static Degree) -> &[Lecture] {
        &self.caches[degree].lectures
    }

    /// Getter method for lazily loading lectures from scraper.
    /// Subsequent method calls return cached lectures.
    pub fn load_lectures(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error> {
        // TODO: Assert that degree is in DEGREES

        if self.caches.get(degree).is_none() {
            let lectures = self.scraper.fetch_lecture_details(degree)?;
            self.caches.insert(degree, LectureCache::new(lectures));
        }

        Ok(&self.caches[degree].lectures)
    }

    /// Attempts to load cached lecture information from specified directory for all supported lectures
    pub fn load_caches<P: AsRef<Path>>(&mut self, directory: &P) -> Vec<std::io::Result<()>> {
        let mut results = Vec::<std::io::Result<()>>::new();
        for degree in Degrees::all() {
            let path = Path::new(directory.as_ref()).join(degree.id).into_boxed_path();
            results.push(match self.load_cache(&path) {
                Ok(cache) => {
                    self.caches.insert(degree, cache);
                    Ok(())
                },
                Err(error) => Err(error)
            });
        }

        results
    }

    /// Attempts to load cached lecture information from a JSON file
    fn load_cache<P: AsRef<Path>>(&mut self, path: &P) -> std::io::Result<LectureCache> {
        let file = open_cache(path)?;
        let cache = serde_json::from_reader(file)?;
        Ok(cache)
    }

    /// Serializes all caches to JSON files and stores them in specified directory
    pub fn save_caches<P: AsRef<Path>>(&mut self, directory: &P) -> Vec<std::io::Result<()>> {
        let mut results = Vec::<std::io::Result<()>>::new();
        for (&degree, cache) in &self.caches {
            let path = Path::new(directory.as_ref()).join(degree.id).into_boxed_path();
            results.push(self.save_cache(&path, cache));
        }

        results
    }

    /// Serializes cache to JSON and writes it to a file
    fn save_cache<P: AsRef<Path>>(&self, path: &P, cache: &LectureCache) -> std::io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer(file, cache)?;
        Ok(())
    }

    /// Serializes cache to JSON formatted with "pretty"-option and writes it to a file
    #[allow(dead_code)]
    fn save_cache_pretty<P: AsRef<Path>>(&self, path: &P, cache: &LectureCache) -> std::io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer_pretty(file, cache)?;
        Ok(())
    }

    /// Deletes cached lectures for all degrees to force scraping them again on next getter call.
    pub fn invalidate_cache(&mut self) {
        self.caches = HashMap::new();
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