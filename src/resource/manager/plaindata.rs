use std::rc::Rc;
use super::super::storage::Storage;
use super::ResourceLoader;
use super::ResourceManager;

pub struct PlainDataLoader { }

impl PlainDataLoader {

    pub fn new() -> Self {
        PlainDataLoader {}
    }

}

impl ResourceLoader for PlainDataLoader {

    type Item = Vec<u8>;
    fn resource_name(&self) -> String {
        "plaindata".to_owned()
    }

    fn load_resource(&self, storage: Rc<Box<Storage>>, path: &str) -> Result<Self::Item, String> {
        storage.load(path)
    }

}

pub type PlainDataManager = ResourceManager<PlainDataLoader>;
