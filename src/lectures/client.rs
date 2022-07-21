use crate::datasource::Error;
use crate::lectures::cache::FSDataSource;
use crate::lectures::entities::Degree;
use crate::repository::LectureRepository;
use super::entities::Lecture;
use super::config::Config;

pub struct LectureClient<'a> {
    repository: LectureRepository<'a>,
}

impl <'a> LectureClient <'a> {
    pub fn from_config(config: Config) -> Self {
        let mut repository = LectureRepository::new(vec![]);
        if let Some(path) = config.get_cache_path() {
            repository.add_source(FSDataSource::new(path.to_string()));
        }

        LectureClient { repository }
    }

    pub fn load_lectures(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error> {
        self.repository.load_lectures(degree)
    }

    /// Returns all lectures that match the given search criteria
    ///
    /// # Arguments
    ///
    /// * `modules` - Search for all lectures matching any of the given module names literally
    pub fn filter_lectures(&mut self, modules: Vec<&str>, degree: &'static Degree) -> Vec<&Lecture> {
        // TODO: Error Handling
        self.repository.load_lectures(degree).unwrap().iter()
            .filter(|lecture| {
                modules.iter().any(|&module| {
                    if let Some(c) = &lecture.categories {
                        c.contains_key(module)
                    } else { false }
                })
            })
            .collect()
    }
}