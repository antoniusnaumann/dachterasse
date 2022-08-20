use crate::{StaticDegree, Lecture};
use async_trait::async_trait;
pub type Error = String;

pub type LoadResult = Result<Vec<Lecture>, Error>;
pub type SaveResult = Result<(), Error>;

#[async_trait]
pub trait ReadOnlyDataSource: Send + Sync {
    async fn load_lectures(&self, degree: &'static StaticDegree) -> LoadResult;
}

#[async_trait]
pub trait ReadWriteDataSource: ReadOnlyDataSource {
    async fn save_lectures(&self, degree: &'static StaticDegree, lectures: &[Lecture]) -> SaveResult;
}
