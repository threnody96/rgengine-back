use std::cell::RefCell;
use std::rc::Rc;
use ::sdl2::render::{ Canvas, Texture, TextureCreator, BlendMode };
use ::sdl2::pixels::PixelFormatEnum::ARGB8888;
use ::sdl2::video::{ Window, WindowContext };

pub struct VirtualCanvas {
    canvas: Rc<RefCell<Canvas<Window>>>,
    vcanvas: RefCell<Texture>,
    texture_creator: Rc<TextureCreator<WindowContext>>,
}

impl VirtualCanvas {

    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self {
        let (w, h) = canvas.borrow().window().size();
        Self::initialize_canvas(&canvas);
        Self {
            canvas: canvas,
            vcanvas: RefCell::new(Self::create_new_vcanvas(texture_creator.clone(), w, h)),
            texture_creator: texture_creator
        }
    }

    fn initialize_canvas(canvas: &Rc<RefCell<Canvas<Window>>>) {
        let mut c = canvas.borrow_mut();
        c.set_blend_mode(BlendMode::Blend);
    }

    fn create_new_vcanvas(texture_creator: Rc<TextureCreator<WindowContext>>, width: u32, height: u32) -> Texture {
        let mut vcanvas = texture_creator.create_texture_target(ARGB8888, width, height).unwrap();
        vcanvas.set_blend_mode(BlendMode::Blend);
        vcanvas
    }

}

pub mod sub_canvas;
pub mod render;
pub mod operation_executer;
