pub trait Storage {
    fn load(&self, path: &str) -> Result<Vec<u8>, String>;
}

pub mod file_storage;
