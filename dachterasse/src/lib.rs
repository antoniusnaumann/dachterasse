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

    #[cfg(feature = "sync")]
    pub mod scrape;

    #[cfg(feature = "sync")]
    pub mod datasource;
    #[cfg(feature = "sync")]
    pub mod repository;

    #[cfg(feature = "sync")]
    pub mod sources {
        mod filesystem_source;
        mod memory_source;
        mod scraper_source;

        pub use filesystem_source::FSDataSource;
        pub use memory_source::InMemoryDataSource;
        pub use scraper_source::ScraperSource;
    }

    #[cfg(feature = "client")]
    pub mod client;
    #[cfg(feature = "client")]
    pub mod config;
}

#[cfg(feature = "async")]
pub mod asynch {
    pub mod lectures {
        pub mod scrape;

        pub mod datasource;
        pub mod repository;

        pub mod sources {
            mod memory_source;
            mod scraper_source;

            pub use memory_source::InMemoryDataSource;
            pub use scraper_source::ScraperSource;
        }
    }
    pub use lectures::*;
}

#[cfg(feature = "sync")]
pub use crate::lectures::client::LectureClient;
#[cfg(feature = "client")]
pub use crate::lectures::config::Config;
pub use crate::lectures::entities::Degree;
pub use crate::lectures::entities::Degrees;
pub use crate::lectures::entities::Lecture;
pub use crate::lectures::entities::StaticDegree;

#[cfg(feature = "client")]
pub use crate::lectures::datasource;
#[cfg(feature = "client")]
pub use crate::lectures::repository;
#[cfg(feature = "client")]
pub use crate::lectures::scrape;
#[cfg(feature = "client")]
pub use crate::lectures::sources;
