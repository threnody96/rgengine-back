use super::storage::Storage;
use std::collections::HashMap;
use std::rc::Rc;

pub trait ResourceLoader<'l, T, L> where T: Storage {

    fn resource_name(&'l self) -> String;
    fn load_resource(&'l self, storage: Rc<T>, path: &str) -> Result<L, String>;

}

pub struct ResourceManager<'l, L, T, R> where T: Storage, R: 'l + ResourceLoader<'l, T, L> {
    cache: HashMap<String, Rc<L>>,
    loader: &'l R,
    storage: Rc<T>
}

impl<'l, L, T, R> ResourceManager<'l, L, T, R> where T: Storage, R: 'l + ResourceLoader<'l, T, L> {

    pub fn new(storage: Rc<T>, loader: &'l R) -> Self {
        Self { cache: HashMap::new(), loader: loader, storage: storage }
    }

    pub fn load(&mut self, path: &str) -> Result<Rc<L>, String> {
        let data = self.cache.get(path).cloned();
        match data {
            None => {
                let storage = self.storage.clone();
                let resource = Rc::new(try!(self.loader.load_resource(storage, path)));
                self.cache.insert(path.to_owned(), resource.clone());
                Ok(resource)
            },
            Some(d) => { Ok(d) }
        }
    }

    pub fn list(&self, dir: Option<&str>) -> Result<Vec<String>, String> {
        self.storage.list(dir)
    }

    pub fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        self.storage.save(path, data)
    }

}

pub mod texture;
pub mod plaindata;
