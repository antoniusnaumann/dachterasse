use crate::asynch::datasource::*;
use crate::{StaticDegree, Lecture};
use async_std::sync::RwLock;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Default)]
pub struct InMemoryDataSource {
    lectures: RwLock<HashMap<&'static StaticDegree, Vec<Lecture>>>,
}

impl InMemoryDataSource {
    pub fn new() -> Self {
        InMemoryDataSource {
            lectures: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ReadOnlyDataSource for InMemoryDataSource {
    async fn load_lectures(&self, degree: &'static StaticDegree) -> LoadResult {
        self.lectures
            .read()
            .await
            .get(degree)
            .cloned()
            .ok_or(format!("No cached lectures for degree {}", degree.name))
    }
}

#[async_trait]
impl ReadWriteDataSource for InMemoryDataSource {
    async fn save_lectures(&self, degree: &'static StaticDegree, lectures: &[Lecture]) -> SaveResult {
        self.lectures
            .write()
            .await
            .insert(degree, Vec::from(lectures));

        Ok(())
    }
}
