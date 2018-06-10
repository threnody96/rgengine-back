use std::rc::Rc;
use super::super::storage::Storage;
use super::ResourceLoader;
use super::ResourceManager;

pub struct PlainDataLoader { }

impl<'l, T: Storage> ResourceLoader<'l, T, Vec<u8>> for PlainDataLoader {

    fn resource_name(&'l self) -> String {
        "plaindata".to_owned()
    }

    fn load_resource(&'l self, storage: Rc<T>, path: &str) -> Result<Vec<u8>, String> {
        storage.load(path)
    }

}

pub type PlainDataManager<'l, T> = ResourceManager<'l, Vec<u8>, T, PlainDataLoader>;
