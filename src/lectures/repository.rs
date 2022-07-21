use std::collections::HashMap;
use crate::datasource::Error;
use crate::lectures::datasource::LectureDataSource;
use crate::lectures::entities::Degree;

use super::entities::Lecture;
use super::scrape::LectureScraper;

/// A lecture repository which uses the file system to persist cached lectures as JSON files
#[derive(Default)]
pub struct LectureRepository<'a> {
    sources: Vec<Box<dyn LectureDataSource + 'a>>,
    // TODO: Scraper should be wrapped in a LectureDataSource
    scraper: LectureScraper,
    caches: HashMap<&'static Degree, Vec<Lecture>>,
}

impl <'a> LectureRepository<'a> {
    /// Create a new repository which serves lectures from its specified data sources
    ///
    /// * `sources` - Data sources from where the repository tries to serve the lecture data.
    /// Loading the data will be attempted in the order in which data sources appear in the vector
    /// until one data source returns a successful result.
    pub fn new(sources: Vec<Box<dyn LectureDataSource>>) -> Self {
        LectureRepository { sources, scraper: LectureScraper::new(), caches: HashMap::new() }
    }

    pub fn add_source(&mut self, source: impl LectureDataSource + 'a) {
        self.sources.push(Box::new(source));
    }

    /// Builder function to easily append additional data sources to repository
    pub fn source(mut self, source: impl LectureDataSource + 'a) -> Self {
        self.add_source(source);
        self
    }

    /// Load lectures from repository data sources
    pub fn load_lectures(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error> {
        if !self.caches.contains_key(degree) {
            if let Some(lectures) = self.sources
                .iter_mut()
                .find_map(|source| source.load_lectures(degree).ok()) {
                self.caches.insert(degree, lectures);
            }
        }

        self.caches
            .get(degree)
            .map(|v| v.as_slice())
            .ok_or_else(|| "Cache did not exist in any data source".to_string())
    }
}