use super::storage::Storage;
use std::collections::HashMap;
use std::rc::Rc;

pub trait ResourceLoader<'l, L> {

    fn resource_name(&'l self) -> String;
    fn load_resource(&'l self, storage: Rc<Box<Storage>>, path: &str) -> Result<L, String>;

}

pub struct ResourceManager<'l, L, R> where R: 'l + ResourceLoader<'l, L> {
    cache: HashMap<String, Rc<L>>,
    loader: &'l R,
    storages: HashMap<String, Rc<Box<Storage>>>
}

impl<'l, L, R> ResourceManager<'l, L, R> where R: 'l + ResourceLoader<'l, L> {

    pub fn new(storages: Vec<Box<Storage>>, loader: &'l R) -> Self {
        Self { cache: HashMap::new(), loader: loader, storages: Self::convert_to_storage_map(storages) }
    }

    fn convert_to_storage_map(storages: Vec<Box<Storage>>) -> HashMap<String, Rc<Box<Storage>>> {
        let mut storage_map: HashMap<String, Rc<Box<Storage>>>  = HashMap::new();
        for storage in storages {
            storage_map.insert(storage.name(), Rc::new(storage));
        }
        storage_map
    }

    pub fn load(&mut self, storage_name: &str, path: &str) -> Result<Rc<L>, String> {
        let cache_key = Self::generate_cache_key(storage_name, path);
        let data = self.cache.get(&cache_key).cloned();
        match data {
            None => {
                let storage = try!(self.storages.get(storage_name).ok_or(format!("storage not found: {}", storage_name))).clone();
                let resource = Rc::new(try!(self.loader.load_resource(storage, path)));
                self.cache.insert(cache_key, resource.clone());
                Ok(resource)
            },
            Some(d) => { Ok(d) }
        }
    }

    pub fn list(&self, storage_name: &str, dir: Option<&str>) -> Result<Vec<String>, String> {
        self.storages.get(storage_name).unwrap().list(dir)
    }

    pub fn save(&self, storage_name: &str, path: &str, data: &Vec<u8>) -> Result<(), String> {
        self.storages.get(storage_name).unwrap().save(path, data)
    }

    fn generate_cache_key(storage_name: &str, path: &str) -> String {
        format!("{}/{}", storage_name, path)
    }

}

pub mod texture;
pub mod plaindata;
