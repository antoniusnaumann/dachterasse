use std::collections::{HashMap, HashSet};
use super::repository::LectureRepository;
use super::entities::Lecture;
use super::config::Config;

pub struct LectureClient {
    repository: LectureRepository,
    config: Config
}

impl LectureClient {
    pub fn with_config(config: Config) -> Self {
        let mut client = LectureClient { repository: LectureRepository::new(), config };
        if let Some(path) = client.config.get_cache_path() {
            if let Err(e) = client.repository.load_cache(&path) {
                eprintln!("Could not load cache: {}", e);
            }
        }
        client
    }

    // TODO: Convert to async function
    /// Loads lectures from repository. Repository might load lectures lazily on first attempt, so this could take a while.
    pub fn lectures(&mut self) -> &[Lecture] {
        self.repository.lectures()
    }

    // TODO: Maybe change repository methods to lectures() and load_lectures() to make mutation more explicit
    pub fn all_lectures(&mut self) -> Vec<&Lecture> {
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
            if let Err(e) = self.repository.save_cache(&path.to_string()) {
                eprintln!("Could not save cache: {}", e);
            }
        }
    }
}