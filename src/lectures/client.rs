use crate::lectures::entities::{Degree, Degrees};
use crate::repository::LectureRepository;
use super::repository::FSLectureRepository;
use super::entities::Lecture;
use super::config::Config;

pub use super::scrape::Error;

pub struct LectureClient {
    repository: FSLectureRepository,
    config: Config
}

impl LectureClient {
    pub fn with_config(config: Config) -> Self {
        let mut client = LectureClient { repository: FSLectureRepository::new(), config };
        if let Some(path) = client.config.get_cache_path() {
            if let Err(e) = client.repository.load_cache(&path) {
                for error in e {
                    eprintln!("Could not load cache: \"{}\"", error);
                    println!("New cache will be created later...");
                }
            }
        }
        client
    }

    pub fn init(&mut self) -> Result<(), Error> {
        for degree in Degrees::all() {
            // TODO: Verbose tag
            println!("Loading lectures for degree {}...", degree.name);
            // TODO: Aggregate errors
            self.load(degree)?;
        }

        Ok(())
    }

    /// Triggers initial load for repository data. Should be called before accessing any lecture data.
    pub fn load(&mut self, degree: &'static Degree) -> Result<&[Lecture], Error> {
        self.repository.load_lectures(degree)
    }

    /// Returns lectures from repository. Can return an empty slice if no lectures were loaded yet
    pub fn lectures(&self, degree: &'static Degree) -> &[Lecture] {
        self.repository.lectures(degree)
    }


    pub fn all_lectures(&self, degree: &'static Degree) -> Vec<&Lecture> {
        self.repository.lectures(degree).iter().collect()
    }

    /// Returns all lectures that match the given search criteria
    ///
    /// # Arguments
    ///
    /// * `modules` - Search for all lectures matching any of the given module names literally
    pub fn filter_lectures(&mut self, modules: Vec<&str>, degree: &'static Degree) -> Vec<&Lecture> {
        self.repository.lectures(degree).iter()
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

impl Drop for LectureClient {
    fn drop(&mut self) {
        if let Some(path) = self.config.get_cache_path() {
            if let Err(e) = self.repository.save_cache(&path) {
                for error in e {
                    eprintln!("Could not save cache: \"{}\"", error);
                }
            }
        }
    }
}