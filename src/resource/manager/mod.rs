use super::storage::Storage;
use std::collections::HashMap;
use std::rc::Rc;

pub trait ResourceLoader<'l> {
    type Item;
    fn resource_name(&'l self) -> String;
    fn load_resource(&'l self, storage: Rc<Box<Storage>>, path: &str) -> Result<Self::Item, String>;
}

pub struct ResourceManager<'l, R> where R: 'l + ResourceLoader<'l> {
    cache: HashMap<String, Rc<R::Item>>,
    loader: &'l R,
}

impl<'l, R> ResourceManager<'l, R> where R: 'l + ResourceLoader<'l> {

    pub fn new(loader: &'l R) -> Self {
        Self { cache: HashMap::new(), loader: loader }
    }

    pub fn load(&mut self, storage: Rc<Box<Storage>>, path: &str) -> Result<Rc<R::Item>, String> {
        let cache_key = Self::generate_cache_key(&storage.name(), path);
        let data = self.cache.get(&cache_key).cloned();
        match data {
            None => {
                let resource = Rc::new(try!(self.loader.load_resource(storage, path)));
                self.cache.insert(cache_key, resource.clone());
                Ok(resource)
            },
            Some(d) => { Ok(d) }
        }
    }

    fn generate_cache_key(storage_name: &str, path: &str) -> String {
        format!("{}/{}", storage_name, path)
    }

}

pub mod texture;
pub mod plaindata;
