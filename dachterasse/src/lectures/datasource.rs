use crate::{StaticDegree, Lecture};

pub type Error = String;

pub type LoadResult = Result<Vec<Lecture>, Error>;
pub type SaveResult = Result<(), Error>;

pub trait ReadOnlyDataSource: Send + Sync {
    fn load_lectures(&self, degree: &'static StaticDegree) -> LoadResult;
}

pub trait ReadWriteDataSource: ReadOnlyDataSource {
    fn save_lectures(&mut self, degree: &'static StaticDegree, lectures: &[Lecture]) -> SaveResult;
}
