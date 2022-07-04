#[derive(Default)]
pub struct Config {
    /// Path to store lecture cache as JSON file. If set to None, no cache is created.
    pub cache_path: Option<String>,
}

impl Config {
    /// Creates a new config without a cache path configured
    pub fn new() -> Self {
        Config { cache_path: None }
    }

    /// Create a config with the default cache path `/cache/lecture_cache.json`
    pub fn with_cache() -> Self {
        Config::new().cache_path("cache/lecture_cache.json".to_string())
    }

    /// Set the cache path for this config
    pub fn cache_path(mut self, path: String) -> Self {
        self.cache_path = Some(path);
        self
    }

    pub fn get_cache_path(&self) -> &Option<String> {
        &self.cache_path
    }
}