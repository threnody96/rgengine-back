use super::Storage;
use util;
use std::fs;
use std::path::PathBuf;
use std::fs::{ read_dir, ReadDir, DirBuilder };
use std::io::{ BufWriter, Write };

const SEPARATOR: char = '/';

pub struct FileStorage {
    name: String,
    storage_dir: PathBuf
}

impl FileStorage {

    pub fn new(name: &str, dir: &str, mkdir: bool) -> Self {
        let mut storage_dir = util::exe_dir();
        storage_dir.push(dir);
        if !storage_dir.exists() && mkdir {
            DirBuilder::new().recursive(true).create(storage_dir.clone()).unwrap();
        }
        if !storage_dir.exists() { panic!(format!("dir not found: {}", storage_dir.to_str().unwrap())); }
        Self { name: name.to_owned(), storage_dir: storage_dir }
    }

    fn convert_to_real_path(&self, path: &str) -> PathBuf {
        let mut real_path = self.storage_dir.clone();
        let paths: Vec<&str> = path.split(SEPARATOR).collect();
        for p in &paths { real_path.push(p); }
        real_path
    }

    fn classify_path(&self, base_dir: Option<String>, entries: ReadDir) -> (Vec<String>, Vec<String>) {
        let mut files: Vec<String> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();
        let prefix = match base_dir {
            None => { "".to_owned() },
            Some(bd) => { format!("{}{}", &bd, SEPARATOR).to_owned() }
        };
        for entry in entries {
            let entry_path = entry.unwrap().path();
            let file_name = entry_path.file_name().unwrap();
            if entry_path.is_file() {
                files.push(format!("{}{}", &prefix, &file_name.to_str().unwrap()));
            } else {
                dirs.push(format!("{}{}", &prefix, &file_name.to_str().unwrap()));
            }
        }
        (dirs, files)
    }

    fn search(&self, dir_paths: Vec<String>, file_paths: Vec<String>) -> Result<Vec<String>, String> {
        let mut next_files = file_paths.clone();
        let mut next_dirs: Vec<String> = vec![];
        for dir_path in &dir_paths {
            let real_path = self.convert_to_real_path(&dir_path);
            if !real_path.exists() { return Err(format!("dir not found: {}", real_path.to_str().unwrap())); }
            let (dirs, files) = self.classify_path(Some(dir_path.to_owned()), read_dir(real_path).unwrap());
            next_files.extend(files.iter().cloned());
            next_dirs.extend(dirs.iter().cloned());
        }
        if next_dirs.len() == 0 { return Ok(next_files); }
        self.search(next_dirs, next_files)
    }

}

impl Storage for FileStorage {

    fn name(&self) -> String {
        self.name.clone()
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let pbuf = self.convert_to_real_path(path);
        Ok(try!(util::load_file(pbuf.as_path())))
    }

    fn list(&self, dir: Option<&str>) -> Result<Vec<String>, String> {
        match dir {
            None => {
                let (dirs, files) = self.classify_path(None, read_dir(self.storage_dir.clone()).unwrap());
                self.search(dirs, files)
            },
            Some(s) => { self.search(vec![s.to_owned()], vec![]) }
        }
    }

    fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        let real_path = self.convert_to_real_path(path);
        let p = real_path.to_str().unwrap();
        let mut f = BufWriter::new(fs::File::create(p).unwrap());
        let r = f.write(data.as_slice());
        if r.is_ok() { Ok(()) } else { Err("Write failed".to_owned()) }
    }

}

