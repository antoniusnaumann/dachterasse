use std::collections::HashMap;
use std::{fs, io};
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

pub trait LectureRepository<T, E> {
    fn new() -> FSLectureRepository {
        FSLectureRepository { scraper: LectureScraper::new(), caches: HashMap::new() }
    }

    fn lectures(&self, degree: &'static Degree) -> &[Lecture];
    fn load_lectures(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error>;
    fn load_cache<P: AsRef<Path>>(&mut self, directory: &P) -> Result<T, E>;
    fn save_cache<P: AsRef<Path>>(&mut self, directory: &P) -> Result<T, E>;
    fn invalidate_cache(&mut self);
}

/// A lecture repository which uses the file system to persist cached lectures as JSON files
#[derive(Default)]
pub struct FSLectureRepository {
    scraper: LectureScraper,
    caches: HashMap<&'static Degree, LectureCache>,
}

impl LectureRepository<(), Vec<io::Error>> for FSLectureRepository {
    /// Get the curently cached lectures without loading them if not present.
    /// Returns an empty slice if no lectures are loaded.
    fn lectures(&self, degree: &'static Degree) -> &[Lecture] {
        if let Some(cache) = &self.caches.get(degree) {
            &cache.lectures
        } else {
            &[]
        }
    }

    /// Getter method for lazily loading lectures from scraper.
    /// Subsequent method calls return cached lectures.
    fn load_lectures(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error> {
        // TODO: Assert that degree is in DEGREES

        if self.caches.get(degree).is_none() {
            let lectures = self.scraper.fetch_lecture_details(degree)?;
            self.caches.insert(degree, LectureCache::new(lectures));
        }

        Ok(&self.caches[degree].lectures)
    }

    /// Attempts to load cached lecture information from specified directory for all supported lectures
    fn load_cache<P: AsRef<Path>>(&mut self, directory: &P) -> Result<(), Vec<io::Error>> {
        let mut errors = Vec::new();
        for degree in Degrees::all() {
            let path = Path::new(directory.as_ref()).join(degree.id).into_boxed_path();
            match self.load_cache_from(&path) {
                Ok(cache) => {
                    self.caches.insert(degree, cache);
                },
                Err(error) => errors.push(error)
            };
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Serializes all caches to JSON files and stores them in specified directory
    fn save_cache<P: AsRef<Path>>(&mut self, directory: &P) -> Result<(), Vec<io::Error>> {
        let mut errors= Vec::new();
        for (&degree, cache) in &self.caches {
            let path = Path::new(directory.as_ref()).join(degree.id).into_boxed_path();
            if let Err(error) = self.save_cache_to(&path, cache) {
                errors.push(error);
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Deletes cached lectures for all degrees to force scraping them again on next getter call.
    fn invalidate_cache(&mut self) {
        self.caches = HashMap::new();
    }
}

impl FSLectureRepository {
    pub fn new() -> Self {
        FSLectureRepository { scraper: LectureScraper::new(), caches: HashMap::new() }
    }

    /// Attempts to load cached lecture information from a JSON file
    fn load_cache_from<P: AsRef<Path>>(&mut self, path: &P) -> io::Result<LectureCache> {
        let file = open_cache(path)?;
        let cache = serde_json::from_reader(file)?;
        Ok(cache)
    }

    /// Serializes cache to JSON and writes it to a file
    fn save_cache_to<P: AsRef<Path>>(&self, path: &P, cache: &LectureCache) -> io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer(file, cache)?;
        Ok(())
    }

    /// Serializes cache to JSON formatted with "pretty"-option and writes it to a file
    #[allow(dead_code)]
    fn save_cache_pretty<P: AsRef<Path>>(&self, path: &P, cache: &LectureCache) -> io::Result<()> {
        let file = create_cache(path)?;
        serde_json::to_writer_pretty(file, cache)?;
        Ok(())
    }
}

fn create_cache<P: AsRef<Path>>(path: &P) -> io::Result<File> {
    create_parent_directory(path)?;
    File::create(ensure_extension(path, "json"))
}

fn open_cache<P: AsRef<Path>>(path: &P) -> io::Result<File> {
    create_parent_directory(path)?;
    File::open(ensure_extension(path, "json"))
}

fn create_parent_directory<P: AsRef<Path>>(path: &P) -> io::Result<()> {
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