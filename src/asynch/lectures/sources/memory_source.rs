use crate::asynch::datasource::*;
use crate::{Degree, Lecture};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Default)]
pub struct InMemoryDataSource {
    lectures: HashMap<&'static Degree, Vec<Lecture>>,
}

impl InMemoryDataSource {
    pub fn new() -> Self {
        InMemoryDataSource {
            lectures: HashMap::new(),
        }
    }
}

#[async_trait]
impl ReadOnlyDataSource for InMemoryDataSource {
    async fn load_lectures(&self, degree: &'static Degree) -> LoadResult {
        self.lectures
            .get(degree)
            .cloned()
            .ok_or(format!("No cached lectures for degree {}", degree.name))
    }
}

#[async_trait]
impl ReadWriteDataSource for InMemoryDataSource {
    async fn save_lectures(&mut self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult {
        self.lectures.insert(degree, Vec::from(lectures));

        Ok(())
    }
}
