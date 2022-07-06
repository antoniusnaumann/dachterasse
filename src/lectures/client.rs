use std::collections::{HashMap, HashSet};
use super::repository::LectureRepository;
use super::entities::Lecture;
use super::config::Config;

pub use super::scrape::Error;

pub struct LectureClient {
    repository: LectureRepository,
    config: Config
}

impl LectureClient {
    pub fn with_config(config: Config) -> Self {
        let mut client = LectureClient { repository: LectureRepository::new(), config };
        if let Some(path) = client.config.get_cache_path() {
            if let Err(e) = client.repository.load_cache(&path) {
                // TODO: Add verbose tag
                eprintln!("Could not load cache: {}", e);
                println!("New cache will be created later...")
            }
        }
        client
    }

    // TODO: Make this return a result if loading was successful
    /// Triggers initial load for repository data. Should be called before accessing any lecture data.
    pub fn init(&mut self) -> Result<&[Lecture], Error> {
        self.repository.load_lectures()
    }

    // TODO: This could be ensured by providing client only through interfaces,
    // TODO: e.g. UnitializedClient and Client. Could also be called in constructor instead but that hides a potentially costly operation
    /// Calls [init] and returns self if init was successful
    pub fn initialized(mut self) -> Result<Self, Error> {
        self.init()?;
        Ok(self)
    }

    /// Returns lectures from repository. Can return an empty slice if no lectures were loaded yet
    pub fn lectures(&self) -> &[Lecture] {
        self.repository.lectures()
    }


    pub fn all_lectures(&self) -> Vec<&Lecture> {
        self.repository.lectures().iter().collect()
    }

    /// Returns all lectures that match the given search criteria
    ///
    /// # Arguments
    ///
    /// * `modules` - Search for all lectures matching any of the given module names literally
    pub fn filter_lectures(&mut self, modules: Vec<&str>) -> Vec<&Lecture> {
        self.repository.lectures().iter()
            .filter(|lecture| {
                modules.iter().any(|&module| {
                    if let Some(c) = &lecture.categories {
                        c.contains_key(module)
                    } else { false }
                })
            })
            .collect()
    }

    // TODO: Cache this somehow
    /// Groups all lectures by category.
    ///
    /// Return format:
    /// {
    ///     module_1_name: {
    ///         category_1_1_name: lecture_a, lecture_b, ...
    ///         ...
    ///     }
    ///     module_2_name: {
    ///         category_2_1_name: lecture_a, lecture_c, ...
    ///         ...
    ///     }
    /// }
    fn group_lectures(&mut self) -> HashMap<String, HashMap<String, HashSet<&Lecture>>> {
        let mut groups: HashMap<String, HashMap<String, HashSet<&Lecture>>> = HashMap::new();

        for lecture in self.repository.lectures() {
            if let Some(modules) = &lecture.categories {
                for (name, categories) in modules {
                    let module = groups.entry(name.to_string()).or_insert_with(HashMap::new);
                    for category in categories {
                        module.entry(category.to_string())
                            .or_insert_with(HashSet::new)
                            .insert(&lecture);
                    }
                }
            }
        }

        groups
    }
}

impl Drop for LectureClient {
    fn drop(&mut self) {
        if let Some(path) = self.config.get_cache_path() {
            if let Err(e) = self.repository.save_cache(path) {
                eprintln!("Could not save cache: {}", e);
            }
        }
    }
}