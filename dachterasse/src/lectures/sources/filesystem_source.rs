use std::fs::File;
use std::{fs, io};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::{Degree, Degrees, Lecture};
use crate::datasource::{LoadResult, SaveResult, ReadWriteDataSource, ReadOnlyDataSource};

pub struct FSDataSource {
    caches: HashMap<&'static Degree, FileSystemCache>
}

impl ReadOnlyDataSource for FSDataSource {
    fn load_lectures(&self, degree: &'static Degree) -> LoadResult {
        self.cache_for_degree(degree)
            .load_lectures()
            .map_err(|err| format!("Could not load cache for degree {} due to {}", degree.id, err))
    }
}

impl ReadWriteDataSource for FSDataSource {
    fn save_lectures(&mut self, degree: &'static Degree, lectures: &[Lecture]) -> SaveResult {
        self.cache_for_degree(degree)
            .save_lectures(lectures)
            .map_err(|err| format!("Could not save cache for degree {} due to {}", degree.id, err))
    }
}

impl FSDataSource {
    pub fn new(path: String) -> Self {
        let mut caches = HashMap::new();

        for degree in Degrees::all() {
            caches.insert(degree, FileSystemCache { path: Path::join(path.as_ref(), degree.id)});
        }

        FSDataSource { caches }
    }

    fn cache_for_degree(&self, degree: &'static Degree) -> &FileSystemCache {
        &self.caches[degree]
    }
}

struct FileSystemCache {
    path: PathBuf
}

impl FileSystemCache {
    pub fn load_lectures(&self) -> io::Result<Vec<Lecture>> {
        load_cache_from(&self.path)
    }

    pub fn save_lectures(&self, lectures: &[Lecture]) -> io::Result<()> {
        save_cache_to(&self.path, lectures)
    }
}

/// Attempts to load cached lecture information from a JSON file
fn load_cache_from<P: AsRef<Path>>(path: &P) -> io::Result<Vec<Lecture>> {
    let file = open_cache(path)?;
    let cache = serde_json::from_reader(file)?;
    Ok(cache)
}

/// Serializes cache to JSON and writes it to a file
fn save_cache_to<P: AsRef<Path>>(path: &P, cache: &[Lecture]) -> io::Result<()> {
    let file = create_cache(path)?;
    serde_json::to_writer(file, cache)?;
    Ok(())
}

/// Serializes cache to JSON formatted with "pretty"-option and writes it to a file
#[allow(dead_code)]
fn save_cache_pretty<P: AsRef<Path>>(path: &P, cache: &[Lecture]) -> io::Result<()> {
    let file = create_cache(path)?;
    serde_json::to_writer_pretty(file, cache)?;
    Ok(())
}

fn create_cache<P: AsRef<Path>>(path: &P) -> io::Result<File> {
    create_parent_directory(path)?;
    File::create(ensure_extension(path, "json"))
}

fn open_cache<P: AsRef<Path>>(path: &P) -> io::Result<File> {
    create_parent_directory(path)?;
    File::open(ensure_extension(path, "json"))
}

fn create_parent_directory<P: AsRef<Path>>(path: &P) -> io::Result<()> {
    if let Some(directories) = path.as_ref().parent() {
        fs::create_dir_all(directories)?;
    }
    Ok(())
}

fn ensure_extension<P: AsRef<Path>>(path: &P, extension: &str) -> Box<Path> {
    let mut buf = path.as_ref().to_path_buf();
    buf.set_extension(extension);
    buf.into_boxed_path()
}