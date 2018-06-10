extern crate sdl2;
extern crate regex;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use storage::Storage;
use self::sdl2::video::WindowContext;
use self::sdl2::surface::Surface;
use self::sdl2::render::{ Texture, TextureCreator };
use self::sdl2::rwops::RWops;
use self::sdl2::image::ImageRWops;
use self::regex::Regex;

pub struct Media<'a, 'b: 'a> {
    texture_creator: TextureCreator<WindowContext>,
    storages: HashMap<String, Box<Storage>>,
    cache: RefCell<MediaCache<'a, 'b>>,
    reg_image_ext: Regex
}

impl<'a, 'b: 'a> Media<'a, 'b> {
    
    pub fn new(texture_creator: TextureCreator<WindowContext>, storage_infos: Vec<(&str, Box<Storage>)>) -> Self {
        let mut storages: HashMap<String, Box<Storage>> = HashMap::new();
        for storage_info in storage_infos {
            let (storage_name, storage) = storage_info;
            storages.insert(storage_name.to_owned(), storage);
        }
        Self {
            texture_creator: texture_creator, 
            storages: storages,
            cache: RefCell::new(MediaCache::new()),
            reg_image_ext: Regex::new("\\.(png|jpg|gif)$").unwrap()
        }
    }

    pub fn load(&'b self, storage_name: &str, path: &str) -> Result<Rc<Vec<u8>>, String> {
        let cached = self.cache.borrow().search_plaindatas(storage_name, path);
        match cached {
            None => {
                let storage = try!(self.storages.get(storage_name).ok_or(format!("storage not found: {}", storage_name)));
                Ok(self.cache.borrow_mut().push_plaindata(storage_name, path, try!(storage.load(path))))
            },
            Some(data) => { Ok(data) }
        }
    }

    pub fn load_surface(&'b self, storage_name: &str, path: &str) -> Result<Rc<Surface<'b>>, String> {
        let cached = self.cache.borrow().search_surfaces(storage_name, path);
        match cached {
            None => {
                let data = try!(self.load(storage_name, path));
                let rwops = RWops::from_bytes(data.as_slice()).unwrap();
                let ext = try!(self.find_image_ext(path).ok_or("unknown image format"));
                let surface = try!(rwops.load_typed(&ext));
                Ok(self.cache.borrow_mut().push_surface(storage_name, path, surface))
            },
            Some(surface) => { Ok(surface) }
        }
    }

    fn load_texture_sub(&'b self, storage_name: &str, path: &str) -> Result<Texture<'a>, String> {
        let surface: Rc<Surface<'b>> = try!(self.load_surface(storage_name, path));
        Ok(self.texture_creator.create_texture_from_surface(*surface).unwrap())
    }

    pub fn load_texture(&'b self, storage_name: &str, path: &str) -> Result<Rc<Texture<'a>>, String> {
        let cached = self.cache.borrow().search_textures(storage_name, path);
        match cached {
            None => {
                let texture = try!(self.load_texture_sub(storage_name, path));
                Ok(self.cache.borrow_mut().push_texture(storage_name, path, texture))
            },
            Some(data) => { Ok(data) }
        }
    }

    fn find_image_ext(&'b self, path: &str) -> Option<String> {
        let cap = self.reg_image_ext.captures(path);
        if cap.is_none() { return None; }
        Some(cap.unwrap().get(1).unwrap().as_str().to_owned())
    }

}

struct MediaCache<'a, 'b: 'a> {
    plaindatas: HashMap<String, Rc<Vec<u8>>>,
    surfaces: HashMap<String, Rc<Surface<'b>>>,
    textures: HashMap<String, Rc<Texture<'a>>>
}

impl<'a, 'b: 'a> MediaCache<'a, 'b> {
    
    fn new() -> Self {
        Self { plaindatas: HashMap::new(), surfaces: HashMap::new(), textures: HashMap::new() }
    }

    fn search_plaindatas(&'b self, storage_name: &str, path: &str) -> Option<Rc<Vec<u8>>> {
        self.plaindatas.get(&Self::generate_key(storage_name, path)).map(|d| d.clone())
    }

    fn push_plaindata(&'b mut self, storage_name: &str, path: &str, data: Vec<u8>) -> Rc<Vec<u8>> {
        self.plaindatas.insert(Self::generate_key(storage_name, path), Rc::new(data));
        self.search_plaindatas(storage_name, path).unwrap()
    }

    fn search_surfaces(&'b self, storage_name: &str, path: &str) -> Option<Rc<Surface<'b>>> {
        self.surfaces.get(&Self::generate_key(storage_name, path)).map(|s| s.clone())
    }

    fn push_surface(&'b mut self, storage_name: &str, path: &str, data: Surface<'b>) -> Rc<Surface<'b>> {
        self.surfaces.insert(Self::generate_key(storage_name, path), Rc::new(data));
        self.search_surfaces(storage_name, path).unwrap()
    }

    fn search_textures(&'b self, storage_name: &str, path: &str) -> Option<Rc<Texture<'a>>> {
        self.textures.get(&Self::generate_key(storage_name, path)).map(|t| t.clone())
    }

    fn push_texture(&'b mut self, storage_name: &str, path: &str, data: Texture<'a>) -> Rc<Texture<'a>> {
        self.textures.insert(Self::generate_key(storage_name, path), Rc::new(data));
        self.search_textures(storage_name, path).unwrap()
    }

    fn generate_key(storage_name: &str, path: &str) -> String {
        format!("{}/{}", storage_name, path)
    }

}
