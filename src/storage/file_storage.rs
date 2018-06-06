use storage::Storage;
use util;
use std::path::{ Path, PathBuf, MAIN_SEPARATOR };
use std::fs::read_dir;

pub struct FileStorage {
    current_path: PathBuf
}

impl FileStorage {

    pub fn new() -> Self {
        Self { current_path: util::exe_dir() }
    }

    fn split_path(&self, path: &str) -> PathBuf {
        let paths: Vec<&str> = path.split(MAIN_SEPARATOR).collect();
        let mut path = self.current_path.clone();
        for p in &paths {
            path.push(p);
        }
        path
    }

    fn search(&self, dir_paths: Vec<PathBuf>, file_paths: Vec<PathBuf>) -> Vec<PathBuf> {
        let mut next_files: Vec<PathBuf> = vec![];
        let mut next_dir_paths: Vec<PathBuf> = vec![];
        for dir_path in &dir_paths {
            let entries = read_dir(dir_path).unwrap();
            for entry in entries {
                let entry_path = entry.unwrap().path();
                let next_path = dir_path.clone().join(entry_path.file_name().unwrap());
                if entry_path.is_file() {
                    next_files.push(entry_path);
                } else {
                    next_dir_paths.push(next_path);
                }
            }
        }
        for fp in &file_paths {
            next_files.push(fp.to_path_buf());
        }
        if next_dir_paths.len() == 0 { return next_files; }
        self.search(next_dir_paths, next_files)
    }

}

impl Storage for FileStorage {

    fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let pbuf = self.split_path(path);
        Ok(try!(util::load_file(pbuf.as_path())))
    }

    fn list(&self, dir: Option<&str>) -> Result<Vec<PathBuf>, String> {
        let d = dir.unwrap_or(".");
        let path = Path::new(d).to_path_buf();
        Ok(self.search(vec![path], vec![]))
    }

    fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        Ok(())
    }

}

