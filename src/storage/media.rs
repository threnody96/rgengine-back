extern crate sdl2;
extern crate regex;

use std::collections::HashMap;
use storage::Storage;
use self::sdl2::video::WindowContext;
use self::sdl2::render::{ Texture, TextureCreator };
use self::sdl2::rwops::RWops;
use self::sdl2::image::ImageRWops;
use self::regex::Regex;

pub struct Media {
    texture_creator: TextureCreator<WindowContext>,
    storages: HashMap<String, Box<Storage>>,
    reg_image_ext: Regex
}

impl Media {
    
    pub fn new(texture_creator: TextureCreator<WindowContext>, storage_infos: Vec<(&str, Box<Storage>)>) -> Self {
        let mut storages: HashMap<String, Box<Storage>> = HashMap::new();
        for storage_info in storage_infos {
            let (storage_name, storage) = storage_info;
            storages.insert(storage_name.to_owned(), storage);
        }
        Self {
            texture_creator: texture_creator, 
            storages: storages,
            reg_image_ext: Regex::new("\\.(png|jpg|gif)$").unwrap()
        }
    }

    pub fn load(&self, storage_name_ref: &str, path_ref: &str) -> Result<Vec<u8>, String> {
        let storage_name = storage_name_ref.to_owned();
        let path = path_ref.to_owned();
        let storage = try!(self.storages.get(&storage_name).ok_or(format!("storage not found: {}", &storage_name)));
        storage.load(&path)
    }

    pub fn load_img(&self, storage_name_ref: &str, path_ref: &str) -> Result<Texture, String> {
        let storage_name = storage_name_ref.to_owned();
        let path = path_ref.to_owned();
        let data = try!(self.load(&storage_name, &path));
        let rwops = RWops::from_bytes(data.as_slice()).unwrap();
        let ext = try!(self.find_image_ext(&path).ok_or("unknown image format"));
        let surface = try!(rwops.load_typed(&ext));
        Ok(self.texture_creator.create_texture_from_surface(surface).unwrap())
    }

    fn find_image_ext(&self, path: &str) -> Option<String> {
        let cap = self.reg_image_ext.captures(path);
        if cap.is_none() { return None; }
        Some(cap.unwrap().get(1).unwrap().as_str().to_owned())
    }

}
