use std::cell::RefCell;
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::PixelFormatEnum::ARGB8888;
use ::sdl2::video::{ Window, WindowContext };

pub struct VirtualCanvas<'l> {
    canvas: &'l RefCell<Canvas<Window>>,
    vcanvas: RefCell<Texture<'l>>,
    texture_creator: &'l TextureCreator<WindowContext>,
}

impl<'l> VirtualCanvas<'l> {

    pub fn new(canvas: &'l RefCell<Canvas<Window>>, texture_creator: &'l TextureCreator<WindowContext>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self::initialize_canvas(canvas);
        Self {
            canvas: canvas,
            vcanvas: RefCell::new(Self::create_new_vcanvas(&texture_creator, w, h)),
            texture_creator: texture_creator
        }
    }

    fn initialize_canvas(canvas: &'l RefCell<Canvas<Window>>) {
        let mut c = canvas.borrow_mut();
        c.set_blend_mode(BlendMode::Blend);
    }

    fn create_new_vcanvas(texture_creator: &'l TextureCreator<WindowContext>, width: u32, height: u32) -> Texture<'l> {
        let mut vcanvas = texture_creator.create_texture_target(ARGB8888, width, height).unwrap();
        vcanvas.set_blend_mode(BlendMode::Blend);
        vcanvas
    }

}

pub mod sub_canvas;
pub mod render;
pub mod operation_executer;
