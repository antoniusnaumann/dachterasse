use crate::{Degree, Lecture};
use async_trait::async_trait;
pub type Error = String;

pub type LoadResult = Result<Vec<Lecture>, Error>;
pub type SaveResult = Result<(), Error>;

#[async_trait]
pub trait ReadOnlyDataSource: Send + Sync {
    async fn load_lectures(&self, degree: &'static Degree) -> LoadResult;
}

#[async_trait]
pub trait ReadWriteDataSource: ReadOnlyDataSource {
    async fn save_lectures(&self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult;
}
