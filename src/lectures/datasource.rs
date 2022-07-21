use crate::{Degree, Lecture};

pub type Error = String;

pub type LoadResult = Result<Vec<Lecture>, Error>;
pub type SaveResult = Result<(), Error>;

pub trait ReadOnlyDataSource {
    fn load_lectures(&self, degree: &'static Degree) -> LoadResult;
}

pub trait ReadWriteDataSource: ReadOnlyDataSource {
    fn save_lectures(&mut self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult;
}