use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, Debug)]
pub struct Lecture {
    pub title: String,
    pub url: String,
    // pub description: Option<String>,
    // TODO: Categories should be structured this way Degree (e.g. ITSE-MA) -> Category (e.g. OSIS) -> Sub-Category (e.g. OSIS-K)
    pub categories: Option<HashMap<String, Vec<String>>>,
}

impl PartialEq for Lecture {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Hash for Lecture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}