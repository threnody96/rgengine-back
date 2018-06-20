use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn exe_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.as_path().parent().unwrap();
    path.to_path_buf()
}

pub fn load_file(path: &Path) -> Result<Vec<u8>, String> {
    let f = try!(File::open(path).map_err(|_| "Failed to read the file: ".to_owned() + &(path.to_str().unwrap())));
    let mut bytes = BufReader::new(f).bytes();
    let mut result: Vec<u8> = vec![];
    while let Some(Ok(b)) = bytes.next() {
        result.push(b);
    }
    Ok(result)
}

pub mod texture;
