use std::collections::HashMap;
use crate::datasource::*;
use crate::{Degree, Lecture};

#[derive(Default)]
pub struct InMemoryDataSource {
    lectures: HashMap<&'static Degree, Vec<Lecture>>,
}

impl InMemoryDataSource {
    pub fn new() -> Self {
        InMemoryDataSource { lectures: HashMap::new() }
    }
}

impl ReadOnlyDataSource for InMemoryDataSource {
    fn load_lectures(&self, degree: &'static Degree) -> LoadResult {
        self.lectures
            .get(degree)
            .cloned()
            .ok_or(format!("No cached lectures for degree {}", degree.name))
    }
}

impl ReadWriteDataSource for InMemoryDataSource {
    fn save_lectures(&mut self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult {
        self.lectures.insert(degree, Vec::from(lectures));

        Ok(())
    }
}