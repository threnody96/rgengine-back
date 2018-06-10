extern crate sdl2;

use std::path::Path;
use std::rc::Rc;
use super::super::storage::Storage;
use super::ResourceLoader;
use super::ResourceManager;
use self::sdl2::render::{ Texture, TextureCreator };
use self::sdl2::rwops::RWops;
use self::sdl2::image::ImageRWops;

impl<'l, T: Storage, W>  ResourceLoader<'l, T, Texture<'l>> for TextureCreator<W> {

    fn resource_name(&'l self) -> String {
        "texture".to_owned()
    }

    fn load_resource(&'l self, storage: Rc<T>, path: &str) -> Result<Texture<'l>, String> {
        let resource = try!(storage.load(path));
        let rwops = RWops::from_bytes(resource.as_slice()).unwrap();
        let ext = Path::new(path).extension().unwrap().to_str().unwrap();
        let surface = try!(rwops.load_typed(ext));
        self.create_texture_from_surface(surface).map_err(|_| "err".to_owned())
    }

}

pub type TextureManager<'l, T, W> = ResourceManager<'l, Texture<'l>, T, TextureCreator<W>>;
