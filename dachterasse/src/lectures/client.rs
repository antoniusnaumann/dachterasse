use super::config::Config;
use super::entities::Lecture;
use crate::datasource::Error;
use crate::lectures::entities::StaticDegree;
use crate::repository::LectureRepository;
use crate::sources::*;
use crate::Degrees;
use std::collections::HashMap;

pub struct LectureClient<'a> {
    repository: LectureRepository<'a>,
    lectures: HashMap<&'static StaticDegree, Vec<Lecture>>,
}

impl<'a> LectureClient<'a> {
    pub fn from_config(config: Config) -> Self {
        let mut repository = LectureRepository::new();
        if let Some(path) = config.get_cache_path() {
            repository.add_source(FSDataSource::new(path.to_string()));
            repository.add_readonly_source(ScraperSource::new());
        }

        let mut lectures = HashMap::new();
        for degree in Degrees::all() {
            lectures.insert(degree, Vec::new());
        }
        LectureClient {
            repository,
            lectures,
        }
    }

    /// Call after creating LectureClient to ensure lectures were loaded
    pub fn init(&mut self) -> Result<(), Error> {
        for degree in Degrees::all() {
            self.load_lectures(degree)?;
        }
        Ok(())
    }

    pub fn initialized(mut self) -> Self {
        let _ = self.init();
        self
    }

    /// Returns lectures if the client was already initialized or an empty slice otherwise
    pub fn lectures(&self, degree: &'static StaticDegree) -> &[Lecture] {
        &self.lectures[degree]
    }

    fn load_lectures(&mut self, degree: &'static StaticDegree) -> Result<&[Lecture], Error> {
        self.lectures
            .insert(degree, self.repository.synchronized_load(degree)?);
        Ok(&self.lectures[degree])
    }

    /// Returns all lectures that match the given search criteria
    ///
    /// # Arguments
    ///
    /// * `modules` - Search for all lectures matching any of the given module names literally
    pub fn filter_lectures(
        &mut self,
        modules: Vec<&str>,
        degree: &'static StaticDegree,
    ) -> Vec<&Lecture> {
        // TODO: Error Handling
        self.lectures[degree]
            .iter()
            .filter(|lecture| {
                modules.iter().any(|&module| {
                    if let Some(c) = &lecture.categories {
                        c.contains_key(module)
                    } else {
                        false
                    }
                })
            })
            .collect()
    }
}
