#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[macro_use]
extern crate serde;

mod lectures {
    pub mod entities;
    pub mod scrape;

    pub mod datasource;
    pub mod repository;

    pub mod sources {
        pub mod filesystem_source;
        pub mod memory_source;
        pub mod scraper_source;

        pub use filesystem_source::FSDataSource;
        pub use memory_source::InMemoryDataSource;
        pub use scraper_source::ScraperSource;
    }

    pub mod client;
    pub mod config;
}

pub use crate::lectures::client::LectureClient;
pub use crate::lectures::config::Config;
pub use crate::lectures::entities::Degree;
pub use crate::lectures::entities::Degrees;
pub use crate::lectures::entities::Lecture;

pub use crate::lectures::datasource;
pub use crate::lectures::repository;
pub use crate::lectures::scrape;
pub use crate::lectures::sources;
