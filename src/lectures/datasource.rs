use crate::{Degree, Lecture};

pub type Error = String;

pub type LoadResult = Result<Vec<Lecture>, Error>;
pub type SaveResult = Result<(), Error>;

pub trait ReadDataSource {
    fn load_lectures(&mut self, degree: &'static Degree) -> LoadResult;
}

pub trait WriteDataSource {
    fn save_lectures(&mut self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult;
}

pub trait LectureDataSource: ReadDataSource + WriteDataSource { }