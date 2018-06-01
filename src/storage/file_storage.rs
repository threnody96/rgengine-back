use storage::Storage;
use util;
use std::path::PathBuf;

pub struct FileStorage {
    current_path: PathBuf
}

impl FileStorage {

    pub fn new() -> Self {
        Self { current_path: util::exe_dir() }
    }

    fn split_path(&self, path: &str) -> PathBuf {
        let paths: Vec<&str> = path.split('/').collect();
        let mut path = self.current_path.clone();
        for p in &paths {
            path.push(p);
        }
        path
    }

}

impl Storage for FileStorage {

    fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let pbuf = self.split_path(path);
        Ok(try!(util::load_file(pbuf.as_path())))
    }

}

