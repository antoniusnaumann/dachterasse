#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod client {
    pub mod load;
    pub mod scrape;
    pub mod repository;
}

pub use crate::client::scrape::LectureScraper;
pub use crate::client::repository::LectureRepository;