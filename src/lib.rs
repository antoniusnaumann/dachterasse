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
    pub mod cache;
    pub mod repository;

    pub mod config;
    pub mod client;
}

pub use crate::lectures::client::LectureClient;
pub use crate::lectures::config::Config;
pub use crate::lectures::entities::Lecture;
pub use crate::lectures::entities::Degrees;
pub use crate::lectures::entities::Degree;

pub use crate::lectures::datasource as datasource;
pub use crate::lectures::repository as repository;pub use crate::lectures::scrape as scrape;
