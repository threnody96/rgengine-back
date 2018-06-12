use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use self::storage::Storage;
use self::manager::texture::TextureManager;
use self::manager::plaindata::{ PlainDataLoader, PlainDataManager };
use ::sdl2::render::{ Texture, TextureCreator };

pub struct Resource<'l, W> where W: 'l {
    storages: HashMap<String, Rc<Box<Storage>>>,
    plaindata: RefCell<PlainDataManager<'l>>,
    texture: RefCell<TextureManager<'l, W>>
}

impl<'l, W> Resource<'l, W> {

    pub fn new(storages:Vec<Box<Storage>>, pl: &'l PlainDataLoader, tc: &'l TextureCreator<W>) -> Self {
        Self {
            storages: Self::convert_to_storage_map(storages),
            plaindata: RefCell::new(PlainDataManager::new(pl)),
            texture: RefCell::new(TextureManager::new(tc))
        }
    }

    pub fn load_plaindata(&self, storage_name: &str, path: &str) -> Result<Rc<Vec<u8>>, String> {
        self.plaindata.borrow_mut().load(try!(self.get_storage(storage_name)), path)
    }

    pub fn load_texture(&self, storage_name: &str, path: &str) -> Result<Rc<Texture<'l>>, String> {
        self.texture.borrow_mut().load(try!(self.get_storage(storage_name)), path)
    }

    pub fn list(&self, storage_name: &str, dir: Option<&str>) -> Result<Vec<String>, String> {
        try!(self.get_storage(storage_name)).list(dir)
    }

    pub fn save(&self, storage_name: &str, path: &str, data: &Vec<u8>) -> Result<(), String> {
        try!(self.get_storage(storage_name)).save(path, data)
    }

    fn convert_to_storage_map(storages: Vec<Box<Storage>>) -> HashMap<String, Rc<Box<Storage>>> {
        let mut storage_map: HashMap<String, Rc<Box<Storage>>>  = HashMap::new();
        for storage in storages {
            storage_map.insert(storage.name(), Rc::new(storage));
        }
        storage_map
    }

    fn get_storage(&self, storage_name: &str) -> Result<Rc<Box<Storage>>, String> {
        let storage = try!(self.storages.get(storage_name).ok_or(format!("storage not found: {}", storage_name))).clone();
        Ok(storage)
    }


}

pub mod storage;
pub mod manager;
