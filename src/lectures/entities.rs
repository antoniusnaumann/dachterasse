use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Eq, Debug, Clone)]
pub struct Lecture {
    pub title: String,
    pub url: String,
    // pub description: Option<String>,
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

pub enum DegreeLevel {
    Bachelor,
    Master
}

// TODO: Serialize to commonly used language abbreviations
pub enum Language {
    German,
    English
}

pub struct Degree {
    /// The section headline string as found in the module overview for a lecture
    pub name: &'static str,
    pub level: DegreeLevel,
    /// Lecture overview site URL for this degree
    pub url: &'static str,
    pub language: Language,
    pub id: &'static str,
}

impl PartialEq for Degree {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Degree { }

impl Hash for Degree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct Degrees { #[allow(dead_code)] no_instance: () }
impl Degrees {
    pub const ITSE_BA: Degree = Degree {
        name: "IT-Systems Engineering BA",
        level: DegreeLevel::Bachelor,
        language: Language::German,
        url: "https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ba.html",
        id: "itse-ba",
    };
    pub const ITSE_MA: Degree = Degree {
        name: "IT-Systems Engineering MA",
        level: DegreeLevel::Master,
        language: Language::German,
        url:"https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html",
        id: "itse-ma",
    };
    pub const DE_MA: Degree = Degree {
        name: "Data Engineering MA",
        level: DegreeLevel::Master,
        language: Language::German,
        url: "https://hpi.de/studium/im-studium/lehrveranstaltungen/data-engineering-ma.html",
        id: "de-ma",
    };
    pub const DH_MA: Degree = Degree {
        name: "Digital Health MA",
        level: DegreeLevel::Master,
        language: Language::English,
        url: "https://hpi.de/studium/im-studium/lehrveranstaltungen/digital-health-ma.html",
        id: "dh-ma",
    };
    pub const CS_MA: Degree = Degree {
        name: "Cybersecurity MA",
        level: DegreeLevel::Master,
        language: Language::English,
        url: "https://hpi.de/studium/im-studium/lehrveranstaltungen/cybersecurity-ma.html",
        id: "cs-ma",
    };
    pub const SSE_MA: Degree = Degree {
        name: "Software Systems Engineering",
        level: DegreeLevel::Master,
        language: Language::English,
        url: "tbd",
        id: "tbd",
    };

    const DEGREES: &'static [Degree] = &[
        Self::ITSE_BA,
        Self::ITSE_MA,
        Self::DE_MA,
        Self::DH_MA,
        Self::CS_MA,
        // Self::SSE_MA
    ];

    pub fn all() -> &'static [Degree]{
        Self::DEGREES
    }
}