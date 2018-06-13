use std::cell::RefCell;
use ::sdl2::render::{ Canvas, Texture, TextureCreator };
use ::sdl2::video::Window;

pub struct VirtualCanvas<'l, T> where T: 'l {
    canvas: &'l RefCell<Canvas<Window>>,
    vcanvas: RefCell<Texture<'l>>,
    texture_creator: &'l TextureCreator<T>,
}

impl<'l, T> VirtualCanvas<'l, T> where T: 'l {

    pub fn new(canvas: &'l RefCell<Canvas<Window>>, texture_creator: &'l TextureCreator<T>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self {
            canvas: canvas,
            vcanvas: RefCell::new(texture_creator.create_texture_target(None, w, h).unwrap()),
            texture_creator: texture_creator
        }
    }

}

pub mod sub_canvas;
pub mod render;
