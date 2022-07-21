use crate::datasource::{Error, ReadOnlyDataSource, ReadWriteDataSource};
use crate::lectures::entities::Degree;

use super::entities::Lecture;

/// A lecture repository which is responsible for keeping lecture information in sync across multiple data sources
#[derive(Default)]
pub struct LectureRepository<'a> {
    sources: Vec<Box<dyn ReadWriteDataSource + 'a>>,
    read_only_sources: Vec<Box<dyn ReadOnlyDataSource + 'a>>,
}

impl <'a> LectureRepository<'a> {
    /// Create a new repository which serves lectures from its specified data sources
    ///
    /// * `sources` - Data sources from where the repository tries to serve the lecture data.
    /// Loading the data will be attempted in the order in which data sources appear in the vector
    /// until one data source returns a successful result.
    pub fn new() -> Self {
        LectureRepository { sources: Vec::new(), read_only_sources: Vec::new() }
    }

    /// Adds a data source to this repository. The repository will synchronize all data sources.
    /// Loading data will be attempted in the order in which data sources are added to the repository
    /// until one data source returns a successful result.
    pub fn add_source(&mut self, source: impl ReadWriteDataSource + 'a) {
        self.sources.push(Box::new(source));
    }

    /// Builder function to easily append additional data sources to repository
    pub fn source(mut self, source: impl ReadWriteDataSource + 'a) -> Self {
        self.add_source(source);
        self
    }

    /// Adds a read-only source to this repository. Read-only sources will only be read if requests to all sources were unsuccessful.
    pub fn add_readonly_source(&mut self, source: impl ReadOnlyDataSource + 'a) {
        self.read_only_sources.push(Box::new(source));
    }

    /// Builder function to easily append additional read-only data sources to repository.
    pub fn readonly_source(mut self, source: impl ReadOnlyDataSource + 'a) -> Self {
        self.add_readonly_source(source);
        self
    }

    /// Load lectures from repository data sources and write them to read-write sources
    pub fn synchronized_load(&mut self, degree: &'static Degree) -> Result<Vec<Lecture>, Error> {
        match self.try_loading(degree) {
            Some(lectures) => {
                for rw in &mut self.sources {
                    let _ = rw.save_lectures(degree, &lectures);
                }
                Ok(lectures)
            }
            None => {
                Err(format!("No source returned lectures for degree {}", degree.name))
            }
        }
    }

    fn try_loading(&self, degree: &'static Degree) -> Option<Vec<Lecture>> {
        self.sources
            .iter()
            .find_map(|source| source.load_lectures(degree).ok())
            .map_or_else(
                || {
                    self.read_only_sources
                        .iter()
                        .find_map(|source| source.load_lectures(degree).ok())

                }, Some)
    }
}