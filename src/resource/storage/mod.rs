pub trait Storage {

    fn name(&self) -> String;
    fn load(&self, path: &str) -> Result<Vec<u8>, String>;
    fn list(&self, dir: Option<&str>) -> Result<Vec<String>, String>;
    fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String>;

}

pub mod file_storage;
